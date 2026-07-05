use anyhow::Context;
use poise::serenity_prelude as serenity;

use crate::{checks::is_owner, context::Ctx, error::BotResult};

/// Shuts down Django (owner only)
#[poise::command(slash_command, check = "is_owner")]
pub async fn shutdown(ctx: Ctx<'_>) -> BotResult {
	ctx.say("Shutting down!")
		.await
		.context("Failed to send message")?;

	ctx.serenity_context()
		.set_presence(None, serenity::OnlineStatus::Offline);

	let shard_manager = ctx
		.framework()
		.shard_manager
		.clone();
	shard_manager
		.shutdown_all()
		.await;

	Ok(())
}
