use anyhow::Context;
use poise::serenity_prelude::AutocompleteChoice;

use crate::{context::Ctx, error::BotResult, services::voice, utils::response};

/// Play media from Jellyfin.
#[poise::command(slash_command, guild_only)]
pub async fn jellyfin(
	ctx: Ctx<'_>,
	#[description = "Media to play."]
	#[autocomplete = "media_autocomplete"]
	media: String,
) -> BotResult {
	ctx.defer()
		.await
		.context("Failed to defer response")?;

	let user = ctx.author();

	let tracks = &ctx
		.data()
		.jellyfin_metadata
		.lock()
		.await
		.tracks;

	let track = tracks.get(&media);

	let track = match track {
		Some(t) => t,
		None => {
			response::error_embed(ctx, "That media does not exist!").await;
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
	let _track = voice::queue_url_ffmpeg(ctx, guild_id, track.audio_url.clone(), true).await?;

	let handler = voice::get_handler(ctx, guild_id).await?;
	let call = handler.lock().await;

	if call.queue().len() != 1 {
		response::success_embed(ctx, "Your song has been added to the queue!").await?;
		return Ok(());
	}

	response::jellyfin_embed(ctx, track).await?;

	Ok(())
}

async fn media_autocomplete(ctx: Ctx<'_>, partial: &str) -> Vec<AutocompleteChoice> {
	let tracks = &ctx
		.data()
		.jellyfin_metadata
		.lock()
		.await
		.tracks;

	tracks
		.iter()
		.filter(|s| {
			s.1.title
				.to_lowercase()
				.contains(&partial.to_lowercase())
		})
		.take(25)
		.map(|s| AutocompleteChoice::new(s.1.title.clone(), s.0.clone()))
		.collect()
}
