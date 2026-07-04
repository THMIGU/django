pub mod ping;
pub mod shutdown;

use crate::{
	commands::{ping::ping, shutdown::shutdown},
	data::Data,
	error::BotError,
};

pub fn commands() -> Vec<poise::Command<Data, BotError>> {
	vec![ping(), shutdown()]
}
