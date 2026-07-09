use anyhow::Context;
use poise::{
	CreateReply,
	serenity_prelude::{Color, CreateEmbed},
};

use crate::{context::Ctx, error::BotResult, services::jellyfin::Track};

const DJANGO_BURGANDY: Color = Color::new(0x650506);

pub async fn error_embed(ctx: Ctx<'_>, message: &str) {
	let embed = CreateEmbed::default()
		.title(format!(":x: {message}"))
		.color(DJANGO_BURGANDY);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await.ok();
}

pub async fn success_embed(ctx: Ctx<'_>, message: &str) -> BotResult {
	let embed = CreateEmbed::default()
		.title(format!(":white_check_mark: {message}"))
		.color(DJANGO_BURGANDY);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send success embed")?;

	Ok(())
}

pub async fn radio_embed(ctx: Ctx<'_>, station: &str) -> BotResult {
	let embed = CreateEmbed::default()
		.title(format!(":notes: Now playing: {station}"))
		.color(DJANGO_BURGANDY);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send radio embed")?;

	Ok(())
}

pub async fn ping_embed(ctx: Ctx<'_>) -> BotResult {
	let embed = CreateEmbed::default()
		.title(":ping_pong: Pong!")
		.color(DJANGO_BURGANDY);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send ping embed")?;

	Ok(())
}

pub async fn shutdown_embed(ctx: Ctx<'_>) -> BotResult {
	let embed = CreateEmbed::default()
		.title(":zzz: Shutting down!")
		.color(DJANGO_BURGANDY);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send shutdown embed")?;

	Ok(())
}

pub async fn jellyfin_embed(ctx: Ctx<'_>, track: &Track) -> BotResult {
	let embed = CreateEmbed::default()
		.title(format!(":notes: Now Playing: {}", &track.title))
		.color(DJANGO_BURGANDY)
		.thumbnail(&track.art_url)
		.fields(vec![
			("Artist(s)", &track.artist, true),
			("Track #", &track.track_number.to_string(), true),
			("Album", &track.album, true),
			("Year", &track.year, true),
			("Sample Rate", &format!("{:.1} kHz", track.sample_rate as f32 / 1000_f32), true),
			("Bit Depth", &track.bit_depth.to_string(), true),
		]);

	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send shutdown embed")?;

	Ok(())
}
