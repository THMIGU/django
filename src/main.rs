mod checks;
mod commands;
mod config;
mod context;
mod data;
mod error;
mod services;

use anyhow::Context;
use poise::serenity_prelude as serenity;
use songbird::SerenityInit;

use crate::{
	commands::commands,
	config::Config,
	data::Data,
	error::{BotResult, on_error},
};

#[tokio::main]
async fn main() -> BotResult {
	let config: Config = Config::load().context("Failed to load config")?;

	let token = config.discord.token.clone();
	let intents = serenity::GatewayIntents::all();

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: commands(),
			on_error: |err| Box::pin(on_error(err)),
			..Default::default()
		})
		.setup(|ctx, ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands)
					.await
					.context("Failed to register commands")?;
				println!("Logged in as {}", ready.user.tag());
				println!("Please ignore Dave. He talks a lot.");

				Ok(Data::new(config))
			})
		})
		.build();

	let mut client = serenity::ClientBuilder::new(&token, intents)
		.framework(framework)
		.register_songbird()
		.await
		.context("Failed to initialize client")?;
	client
		.start()
		.await
		.context("Failed to start client")?;

	println!("Shutting down");

	Ok(())
}
