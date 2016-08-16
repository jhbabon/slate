use std::result::Result;
use errors::{SlateError, CommandError};
use message::Message;

pub type SlateResult<T> = Result<T, SlateError>;
pub type CommandResult = Result<Option<Message>, CommandError>;
