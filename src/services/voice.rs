use std::sync::Arc;

use anyhow::Context;
use poise::serenity_prelude::{ChannelId, GuildId};
use reqwest::Client;
use songbird::{Call, Songbird, input::HttpRequest, tracks::TrackHandle};
use tokio::sync::Mutex;

use crate::{context::Ctx, error::BotResult};

type Manager = Arc<Songbird>;
type Handler = Arc<Mutex<Call>>;

pub async fn get_manager(ctx: Ctx<'_>) -> Manager {
	songbird::get(ctx.serenity_context())
		.await
		.expect("Songbird not initialized")
		.clone()
}

pub async fn get_handler(ctx: Ctx<'_>, guild_id: GuildId) -> BotResult<Handler> {
	let manager = get_manager(ctx).await;
	let handler = manager
		.get(guild_id)
		.context("Call not found")?;

	Ok(handler)
}

pub async fn join(ctx: Ctx<'_>, guild_id: GuildId, channel_id: ChannelId) -> BotResult {
	if let Ok(handler) = get_handler(ctx, guild_id).await {
		let call = handler.lock().await;
		let current_channel_id = call
			.current_channel()
			.expect("Channel ID not found");

		if current_channel_id == channel_id.into() {
			return Ok(());
		}

		call.queue().stop();
	}

	let manager = get_manager(ctx).await;

	manager
		.join(guild_id, channel_id)
		.await
		.context("Failed to join voice channel")?;

	Ok(())
}

pub async fn leave(ctx: Ctx<'_>, guild_id: GuildId) -> BotResult {
	let manager = get_manager(ctx).await;

	manager
		.leave(guild_id)
		.await
		.context("Failed to leave voice channel")?;

	Ok(())
}

pub async fn play_url(
	client: Client,
	handler: Handler,
	stream_url: String,
) -> BotResult<TrackHandle> {
	let mut call = handler.lock().await;
	call.set_bitrate(songbird::driver::Bitrate::Max);
	call.deafen(true)
		.await
		.context("Failed to deafen")?;

	let input = HttpRequest::new(client, stream_url.clone());
	let track_handle = call
		.enqueue(input.into())
		.await;

	Ok(track_handle)
}
