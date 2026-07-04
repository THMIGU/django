mod config;

use std::fs;

use anyhow::{Context, Result};
use poise::serenity_prelude as serenity;

use crate::config::Config;

struct Data {}
type Ctx<'a> = poise::Context<'a, Data, anyhow::Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Ctx<'_>) -> Result<()> {
	let _user = ctx.author();
	ctx.say("Pong!").await?;

	Ok(())
}

#[poise::command(slash_command)]
async fn shutdown(ctx: Ctx<'_>) -> Result<()> {
	let _user = ctx.author();
	ctx.say("Shutting down!")
		.await?;

	let shard_manager = ctx
		.framework()
		.shard_manager
		.clone();
	let shard_messenger = &ctx.serenity_context().shard;

	shard_messenger.set_status(serenity::OnlineStatus::Offline);
	shard_manager
		.shutdown_all()
		.await;

	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	let config_str = fs::read_to_string("config.toml").context("Failed to read config.toml")?;
	let config: Config = toml::from_str(&config_str).context("Failed to parse config.toml")?;

	let token = config.token;
	let intents = serenity::GatewayIntents::all();

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![ping(), shutdown()],
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;

				println!("Ready!");
				Ok(Data {})
			})
		})
		.build();

	let mut client = serenity::ClientBuilder::new(&token, intents)
		.framework(framework)
		.await?;
	client.start().await?;

	Ok(())
}
