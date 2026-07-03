mod config;

use anyhow::Result;
use serenity::{
	Client,
	all::{Context, EventHandler, GatewayIntents, Message},
	async_trait,
};

use crate::config::Config;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content == "!ping" {
			if let Err(why) = msg
				.channel_id
				.say(&ctx.http, "Pong!")
				.await
			{
				println!("Error sending message: {why:?}")
			}
		}
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	let config: Config = toml::from_str(include_str!("../config.toml"))?;

	let token = config.token;
	let intents = GatewayIntents::all();

	let mut client = Client::builder(&token, intents)
		.event_handler(Handler)
		.await?;

	client.start().await?;

	Ok(())
}
