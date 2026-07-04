use std::fs;

use anyhow::Context;
use serde::Deserialize;

use crate::error::BotResult;

#[derive(Deserialize)]
pub struct Config {
	pub token: String,
	pub owner_id: u64,
}

impl Config {
	pub fn load() -> BotResult<Self> {
		let config_str = fs::read_to_string("config.toml").context("Failed to read config.toml")?;
		let config: Config = toml::from_str(&config_str).context("Failed to parse config.toml")?;

		Ok(config)
	}
}
