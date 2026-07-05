use anyhow::Error;
use poise::FrameworkError;

use crate::data::Data;

pub type BotError = Error;
pub type BotResult<T = ()> = Result<T, BotError>;

pub async fn on_error(error: FrameworkError<'_, Data, BotError>) {
	match error {
		FrameworkError::Command {
			error,
			ctx,
			..
		} => {
			eprintln!("{:#}", error);

			let _ = ctx
				.say("An error occured while executing this command!")
				.await;
		}
		FrameworkError::CommandCheckFailed {
			error,
			ctx,
			..
		} => {
			if let Some(error) = error {
				eprintln!("{:#}", error);
			}

			let _ = ctx
				.say("You cannot use this command!")
				.await;
		}
		other => {
			eprintln!("{:#}", other);
		}
	};
}
