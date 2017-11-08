use serde_json;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum SlateError {
    IO(io::Error),
    JSON(serde_json::Error),
}

impl fmt::Display for SlateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SlateError::IO(ref err) => write!(f, "{}", err),
            SlateError::JSON(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for SlateError {
    fn description(&self) -> &str {
        match *self {
            SlateError::IO(ref err) => err.description(),
            SlateError::JSON(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SlateError::IO(ref err) => Some(err),
            SlateError::JSON(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for SlateError {
    fn from(err: io::Error) -> SlateError {
        SlateError::IO(err)
    }
}

impl From<serde_json::Error> for SlateError {
    fn from(err: serde_json::Error) -> SlateError {
        SlateError::JSON(err)
    }
}

#[derive(Debug)]
pub enum CommandError {
    IO(io::Error),
    Slate(SlateError),
    Argument(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::IO(ref err) => write!(f, "{}", err),
            CommandError::Slate(ref err) => write!(f, "{}", err),
            CommandError::Argument(ref string) => write!(f, "{}", string),
        }
    }
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::IO(ref err) => err.description(),
            CommandError::Slate(ref err) => err.description(),
            CommandError::Argument(ref string) => string,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CommandError::IO(ref err) => Some(err),
            CommandError::Slate(ref err) => Some(err),
            CommandError::Argument(_) => None,
        }
    }
}

impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> CommandError {
        CommandError::IO(err)
    }
}

impl From<SlateError> for CommandError {
    fn from(err: SlateError) -> CommandError {
        CommandError::Slate(err)
    }
}

impl From<String> for CommandError {
    fn from(err: String) -> CommandError {
        CommandError::Argument(err)
    }
}
