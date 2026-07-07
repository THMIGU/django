mod jellyfin;
mod ping;
mod radio;
mod shutdown;
mod stop;

use crate::{
	commands::{jellyfin::jellyfin, ping::ping, radio::radio, shutdown::shutdown, stop::stop},
	data::Data,
	error::BotError,
};

pub fn commands() -> Vec<poise::Command<Data, BotError>> {
	vec![ping(), shutdown(), stop(), radio(), jellyfin()]
}
