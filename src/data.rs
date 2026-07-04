use crate::config::Config;

pub struct Data {
	pub config: Config,
}

impl Data {
	pub fn new(config: Config) -> Self {
		Data {
			config,
		}
	}
}
