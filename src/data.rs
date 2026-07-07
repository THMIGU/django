use reqwest::Client;
use tokio::sync::Mutex;

use crate::{config::Config, services::jellyfin::JellyfinMetadata};

pub struct Data {
	pub config: Config,
	pub http_client: Client,
	pub jellyfin_metadata: Mutex<JellyfinMetadata>,
}

impl Data {
	pub fn new(config: Config) -> Self {
		Data {
			config,
			http_client: Client::new(),
			jellyfin_metadata: JellyfinMetadata::default().into(),
		}
	}
}
