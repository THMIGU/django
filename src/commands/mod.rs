mod ping;
mod play;
mod shutdown;
mod stop;

use crate::{
	commands::{ping::ping, play::play, shutdown::shutdown, stop::stop},
	data::Data,
	error::BotError,
};

pub fn commands() -> Vec<poise::Command<Data, BotError>> {
	vec![ping(), shutdown(), play(), stop()]
}
