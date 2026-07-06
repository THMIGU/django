use crate::{context::Ctx, error::BotResult, services::voice, utils::response};

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
		response::error_embed(ctx, "You are not in a voice channel!").await;
		return Ok(());
	};

	{
		let Some(handler) = voice::get_handler(ctx, guild_id)
			.await
			.ok()
		else {
			response::error_embed(ctx, "There is nothing to stop!").await;
			return Ok(());
		};
		let mut call = handler.lock().await;

		call.stop();
	}

	voice::leave(ctx, guild_id).await?;
	response::success_embed(ctx, "All playback stopped!").await?;

	Ok(())
}
