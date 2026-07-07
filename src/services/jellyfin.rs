use std::collections::HashMap;

use crate::{context::Ctx, error::BotResult};
use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
struct JellyfinResponse {
	#[serde(rename = "Items", default)]
	items: Vec<JellyfinItem>,
}

#[derive(Debug, Deserialize, Default)]
struct JellyfinItem {
	#[serde(rename = "MediaSources", default)]
	media_sources: Vec<MediaSource>,
	#[serde(rename = "Id")]
	item_id: String,
	#[serde(rename = "Name", default)]
	title: String,
	#[serde(rename = "Artists", default)]
	artists: Vec<String>,
	#[serde(rename = "PremiereDate", default)]
	release: String,
	#[serde(rename = "Album", default)]
	album: String,
	#[serde(rename = "IndexNumber", default)]
	track_number: u32,
	#[serde(rename = "ImageTags", default)]
	image_tags: ImageTags,
}

#[derive(Debug, Deserialize, Default)]
struct MediaSource {
	#[serde(rename = "MediaStreams", default)]
	media_streams: Vec<MediaStream>,
}

#[derive(Debug, Deserialize, Default)]
struct MediaStream {
	#[serde(rename = "Type", default)]
	stream_type: String,
	#[serde(rename = "BitDepth", default)]
	bit_depth: u32,
	#[serde(rename = "SampleRate", default)]
	sample_rate: u32,
}

#[derive(Debug, Deserialize, Default)]
struct ImageTags {
	#[serde(rename = "Primary", default)]
	primary: String,
}

#[derive(Default, Debug)]
pub struct JellyfinMetadata {
	pub tracks: HashMap<String, Track>,
	pub albums: HashMap<String, Album>,
}

#[derive(Debug)]
pub struct Track {
	pub title: String,
	pub artist: String,
	pub year: String,
	pub album: String,
	pub track_number: u32,
	pub bit_depth: u32,
	pub sample_rate: u32,
	pub art_url: String,
	pub audio_url: String,
}

impl Track {
	fn from_item(api_url: &str, item: JellyfinItem) -> Self {
		let media_sources = item.media_sources;
		let media_stream = media_sources
			.first()
			.and_then(|source| {
				source
					.media_streams
					.iter()
					.find(|stream| stream.stream_type == "Audio")
			});

		let bit_depth = media_stream
			.map(|s| s.bit_depth)
			.unwrap_or(0);
		let sample_rate = media_stream
			.map(|s| s.sample_rate)
			.unwrap_or(0);

		let year: String = item
			.release
			.chars()
			.take(4)
			.collect();
		let artist = item.artists.join(", ");

		let item_id = item.item_id;
		let art_tag = item.image_tags.primary;

		let art_url = format!(
			"{}/Items/{}/Images/Primary?maxHeight=320&quality=90&tag={}",
			api_url, item_id, art_tag
		);
		let audio_url = format!("{}/Audio/{}/stream?static=true", api_url, item_id);

		Self {
			title: item.title,
			artist,
			year,
			album: item.album,
			track_number: item.track_number,
			bit_depth,
			sample_rate,
			art_url,
			audio_url,
		}
	}
}

#[derive(Debug)]
pub struct Album {
	pub title: String,
	pub artist: String,
	pub tracks: Vec<String>,
}

pub async fn get_metadata(ctx: Ctx<'_>) -> BotResult<JellyfinMetadata> {
	let client = &ctx.data().http_client;
	let config = &ctx.data().config;

	let api_url = &config.jellyfin.api_url;
	let api_key = &config.jellyfin.api_key;
	let user_id = &config.jellyfin.user_id;

	let url = format!("{api_url}/Users/{user_id}/Items");

	let res = client
		.get(url)
		.header("X-Emby-Token", api_key)
		.query(&[
			("IncludeItemTypes", "Audio"),
			("Recursive", "true"),
			("SortBy", "IndexNumber"),
			("SortOrder", "Ascending"),
			("Fields", "MediaSources,PremiereDate,Artists,Album,ImageTags"),
		])
		.send()
		.await
		.context("Failed to retrieve Jellyfin metadata")?
		.json::<JellyfinResponse>()
		.await?;

	let mut jellyfin_metadata = JellyfinMetadata::default();

	for item in res.items {
		if item.title.is_empty() {
			continue;
		}

		let item_id = item.item_id.clone();
		let track = Track::from_item(api_url, item);

		jellyfin_metadata
			.albums
			.entry(track.album.clone())
			.or_insert_with(|| Album {
				title: track.album.clone(),
				artist: track.artist.clone(),
				tracks: vec![],
			})
			.tracks
			.push(item_id.clone());

		jellyfin_metadata
			.tracks
			.insert(item_id, track);
	}

	Ok(jellyfin_metadata)
}

pub async fn cache_metadata(ctx: Ctx<'_>) -> BotResult {
	let metadata = get_metadata(ctx).await?;

	let mut jellyfin_metadata = ctx
		.data()
		.jellyfin_metadata
		.lock()
		.await;
	*jellyfin_metadata = metadata;

	Ok(())
}
