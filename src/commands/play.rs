use std::time::Duration;

use anyhow::Context;
use songbird::input::HttpRequest;

use crate::{context::Ctx, error::BotResult};

#[poise::command(slash_command, guild_only)]
pub async fn play(ctx: Ctx<'_>) -> BotResult {
	let user = ctx.author();
	let guild_id = ctx
		.guild_id()
		.expect("Guild not found");

	let channel = {
		let guild = ctx
			.guild()
			.expect("Guild not found");

		guild
			.voice_states
			.get(&user.id)
			.and_then(|vs| vs.channel_id)
	};

	let Some(channel) = channel else {
		ctx.say("You are not in a voice channel!")
			.await
			.context("Failed to send message")?;
		return Ok(());
	};

	let manager = songbird::get(ctx.serenity_context())
		.await
		.expect("Songbird not initialized")
		.clone();

	manager
		.join(guild_id, channel)
		.await
		.context("Failed to join voice channel")?;
	ctx.say("Joined!")
		.await
		.context("Failed to send message")?;

	let handler = manager
		.get(guild_id)
		.expect("Call not found");
	let mut call = handler.lock().await;

	let stream_url = "https://live.hunter.fm/lofi_high".to_string();
	let input = HttpRequest::new(reqwest::Client::new(), stream_url);

	let _track = call.play_input(input.into());

	Ok(())
}
