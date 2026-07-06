use poise::serenity_prelude as serenity;

use crate::{checks::is_owner, context::Ctx, error::BotResult, utils::response};

/// Shuts down Django (owner only).
#[poise::command(slash_command, check = "is_owner")]
pub async fn shutdown(ctx: Ctx<'_>) -> BotResult {
	response::shutdown_embed(ctx).await?;

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
