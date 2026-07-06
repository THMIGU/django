use anyhow::Context;
use poise::serenity_prelude::AutocompleteChoice;

use crate::{context::Ctx, error::BotResult, services::voice, utils::response};

/// Play an internet radio.
#[poise::command(slash_command, guild_only)]
pub async fn radio(
	ctx: Ctx<'_>,
	#[description = "Station to play."]
	#[autocomplete = "station_autocomplete"]
	station_name: String,
) -> BotResult {
	ctx.defer()
		.await
		.context("Failed to defer response")?;

	let user = ctx.author();

	let stations = &ctx
		.data()
		.config
		.django
		.stations;

	let station = stations
		.iter()
		.find(|s| s.name == station_name);

	let station = match station {
		Some(s) => s,
		None => {
			response::error_embed(ctx, "That station does not exist!").await;
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
		response::error_embed(ctx, "You are not in a voice channel!").await;
		return Ok(());
	};

	voice::join(ctx, guild_id, channel_id).await?;
	response::radio_embed(ctx, &station.name).await?;

	let handler = voice::get_handler(ctx, guild_id).await?;
	let client = ctx.data().http_client.clone();

	let _track = voice::play_url(client, handler, station.url.clone()).await?;

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
