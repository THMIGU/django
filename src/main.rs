mod checks;
mod commands;
mod config;
mod context;
mod data;
mod error;

use anyhow::Result;
use poise::serenity_prelude as serenity;

use crate::{commands::commands, config::Config, data::Data, error::on_error};

#[tokio::main]
async fn main() -> Result<()> {
	let config: Config = Config::load()?;

	let token = config.token.clone();
	let intents = serenity::GatewayIntents::all();

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: commands(),
			on_error: |err| Box::pin(on_error(err)),
			..Default::default()
		})
		.setup(|ctx, ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				println!("Logged in as {}", ready.user.tag());

				Ok(Data {
					config,
				})
			})
		})
		.build();

	let mut client = serenity::ClientBuilder::new(&token, intents)
		.framework(framework)
		.await?;
	client.start().await?;

	println!("Shutting down!");

	Ok(())
}
