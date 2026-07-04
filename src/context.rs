use crate::{data::Data, error::BotError};

pub type Ctx<'a> = poise::Context<'a, Data, BotError>;
