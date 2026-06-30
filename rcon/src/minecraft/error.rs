//! Common error types for Minecraft `RCON`.

use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// Common Minecraft `RCON` errors.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// A UTF-8 error occurred.
    Utf8(FromUtf8Error),
    /// The command is incomplete or unknown.
    UnknownOrIncompleteCommand(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::Utf8(error) => error.fmt(f),
            Self::UnknownOrIncompleteCommand(command) => write!(f, "Unknown command: {command}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Utf8(error) => Some(error),
            Self::UnknownOrIncompleteCommand(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::Io(error) => Self::Io(error),
            crate::Error::Utf8(error) => Self::Utf8(error),
        }
    }
}
