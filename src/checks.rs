use crate::{context::Ctx, error::BotResult};

pub async fn is_owner(ctx: Ctx<'_>) -> BotResult<bool> {
	Ok(ctx.author().id
		== ctx
			.data()
			.config
			.discord
			.owner_id)
}
