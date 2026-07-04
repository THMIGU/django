mod ping;
mod play;
mod shutdown;

use crate::{
	commands::{ping::ping, play::play, shutdown::shutdown},
	data::Data,
	error::BotError,
};

pub fn commands() -> Vec<poise::Command<Data, BotError>> {
	vec![ping(), shutdown(), play()]
}
