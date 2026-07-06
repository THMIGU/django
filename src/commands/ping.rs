use anyhow::Context;

use crate::{context::Ctx, error::BotResult};

/// Responds with "Pong!" when active.
#[poise::command(slash_command)]
pub async fn ping(ctx: Ctx<'_>) -> BotResult {
	ctx.say("Pong!")
		.await
		.context("Failed to send message")?;

	Ok(())
}
