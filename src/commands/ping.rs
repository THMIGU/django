use crate::{context::Ctx, error::BotResult, utils::response};

/// Responds with "Pong!" when active.
#[poise::command(slash_command)]
pub async fn ping(ctx: Ctx<'_>) -> BotResult {
	response::ping_embed(ctx).await?;

	Ok(())
}
