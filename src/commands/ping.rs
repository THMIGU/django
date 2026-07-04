use crate::{context::Ctx, error::BotResult};

#[poise::command(slash_command)]
pub async fn ping(ctx: Ctx<'_>) -> BotResult {
	ctx.say("Pong!").await?;

	Ok(())
}
