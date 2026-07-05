use anyhow::Context;

use crate::{context::Ctx, error::BotResult, services::voice};

/// Joins your VC and plays Hunter.FM
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

	let Some(channel_id) = channel else {
		ctx.say("You are not in a voice channel!")
			.await
			.context("Failed to send message")?;
		return Ok(());
	};

	voice::join(ctx, guild_id, channel_id).await?;
	ctx.say("Joined!")
		.await
		.context("Failed to send message")?;

	let stream_url = "https://live.hunter.fm/lofi_high".to_string();
	let handler = voice::get_handler(ctx, guild_id).await?;

	let _track = voice::play_url(handler, stream_url);

	Ok(())
}
