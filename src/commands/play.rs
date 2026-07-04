use anyhow::Context;

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
		.await?;

	Ok(())
}
