use anyhow::Context;

use crate::{context::Ctx, error::BotResult, services::voice};

/// Stop all playback.
#[poise::command(slash_command, guild_only)]
pub async fn stop(ctx: Ctx<'_>) -> BotResult {
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

	if channel.is_none() {
		ctx.say("You are not in a voice channel!")
			.await
			.context("Failed to send message")?;
		return Ok(());
	};

	{
		let handler = voice::get_handler(ctx, guild_id).await?;
		let mut call = handler.lock().await;

		call.stop();
	}

	voice::leave(ctx, guild_id).await?;
	ctx.say("Stopped!")
		.await
		.context("Failed to send message")?;

	Ok(())
}
