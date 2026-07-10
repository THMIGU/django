use std::{
	process::{Command, Stdio},
	sync::Arc,
};

use anyhow::Context;
use poise::serenity_prelude::{ChannelId, GuildId, async_trait};
use songbird::{
	Call, Event, EventContext, EventHandler, Songbird, TrackEvent,
	input::{ChildContainer, HttpRequest, Input, RawAdapter},
	tracks::TrackHandle,
};
use symphonia::core::io::ReadOnlySource;
use tokio::sync::Mutex;

use crate::{context::Ctx, error::BotResult};

type Manager = Arc<Songbird>;
type Handler = Arc<Mutex<Call>>;

struct TrackEndNotifier {
	pub manager: Manager,
	pub guild_id: GuildId,
}

#[async_trait]
impl EventHandler for TrackEndNotifier {
	async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
		let EventContext::Track(_) = ctx else {
			return None;
		};

		if let Some(handler) = self
			.manager
			.get(self.guild_id)
		{
			let call = handler.lock().await;

			if !call.queue().is_empty() {
				return None;
			}

			drop(call);

			self.manager
				.remove(self.guild_id)
				.await
				.ok();
		}

		None
	}
}

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
		if let Some(current_channel_id) = call.current_channel() {
			if current_channel_id == channel_id.into() {
				return Ok(());
			}
		};

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
		.remove(guild_id)
		.await
		.context("Failed to leave voice channel")?;

	Ok(())
}

pub async fn _play_url(
	ctx: Ctx<'_>,
	guild_id: GuildId,
	stream_url: String,
) -> BotResult<TrackHandle> {
	let manager = get_manager(ctx).await;
	let handler = manager
		.get(guild_id)
		.context("Call not found")?;

	let mut call = handler.lock().await;
	call.set_bitrate(songbird::driver::Bitrate::Max);
	call.deafen(true)
		.await
		.context("Failed to deafen")?;

	if call.queue().is_empty() {
		call.add_global_event(
			Event::Track(TrackEvent::End),
			TrackEndNotifier {
				manager,
				guild_id,
			},
		);
	}

	let client = ctx.data().http_client.clone();
	let input = HttpRequest::new(client, stream_url.clone());
	let track_handle = call
		.enqueue(input.into())
		.await;

	Ok(track_handle)
}

pub async fn queue_url_ffmpeg(
	ctx: Ctx<'_>,
	guild_id: GuildId,
	stream_url: String,
	realtime: bool,
) -> BotResult<TrackHandle> {
	let manager = get_manager(ctx).await;
	let handler = manager
		.get(guild_id)
		.context("Call not found")?;

	let mut call = handler.lock().await;
	call.set_bitrate(songbird::driver::Bitrate::Max);
	call.deafen(true)
		.await
		.context("Failed to deafen")?;

	if call.queue().is_empty() {
		call.add_global_event(
			Event::Track(TrackEvent::End),
			TrackEndNotifier {
				manager,
				guild_id,
			},
		);
	}

	let mut prefix_args =
		vec!["-reconnect", "1", "-reconnect_streamed", "1", "-reconnect_delay_max", "5"];

	if realtime {
		prefix_args.push("-re");
	}

	let child = Command::new("ffmpeg")
		.args(prefix_args)
		.args(["-i", &stream_url, "-vn", "-f", "f32le", "-ar", "192000", "-ac", "2", "pipe:1"])
		.stdin(Stdio::null())
		.stderr(Stdio::null())
		.stdout(Stdio::piped())
		.spawn()?;

	let stream = ChildContainer::from(child);
	let source = ReadOnlySource::new(stream);
	let input: Input = RawAdapter::new(source, 192000, 2).into();

	let track_handle = call
		.enqueue(input.into())
		.await;

	Ok(track_handle)
}
