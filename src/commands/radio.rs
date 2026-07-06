use anyhow::Context;
use poise::serenity_prelude::AutocompleteChoice;

use crate::{context::Ctx, error::BotResult, services::voice};

/// Play an internet radio.
#[poise::command(slash_command, guild_only)]
pub async fn radio(
	ctx: Ctx<'_>,
	#[description = "Station to play."]
	#[autocomplete = "station_autocomplete"]
	station_name: String,
) -> BotResult {
	let user = ctx.author();

	let stations = &ctx
		.data()
		.config
		.django
		.stations;

	let station = stations
		.iter()
		.find(|s| s.name == station_name);

	let station_url = match station {
		Some(s) => &s.url,
		None => {
			ctx.reply("That station does not exist!")
				.await
				.context("Failed to send message")?;
			return Ok(());
		}
	};

	let guild_id = ctx
		.guild_id()
		.expect("Guild not found");

	let channel = {
		let guild = ctx
			.guild()
			.expect("Guild not found");

		guild
			.voice_states
			.get(&user.id)
			.and_then(|vs| vs.channel_id)
	};

	let Some(channel_id) = channel else {
		ctx.reply("You are not in a voice channel!")
			.await
			.context("Failed to send message")?;
		return Ok(());
	};

	voice::join(ctx, guild_id, channel_id).await?;
	ctx.say("Joined!")
		.await
		.context("Failed to send message")?;

	let handler = voice::get_handler(ctx, guild_id).await?;
	let client = ctx.data().http_client.clone();

	let _track = voice::play_url(client, handler, station_url.clone()).await?;

	Ok(())
}

async fn station_autocomplete(ctx: Ctx<'_>, partial: &str) -> Vec<AutocompleteChoice> {
	let stations = &ctx
		.data()
		.config
		.django
		.stations;

	stations
		.iter()
		.filter(|s| {
			s.name
				.to_lowercase()
				.contains(&partial.to_lowercase())
		})
		.take(25)
		.map(|s| AutocompleteChoice::new(s.name.clone(), s.name.clone()))
		.collect()
}
