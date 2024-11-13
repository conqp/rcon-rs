//! Common error types.

use std::fmt::Display;
use std::string::FromUtf8Error;

/// Represents either an I/O error or a UTF-8 error.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// A UTF-8 parsing error occurred.
    Utf8(FromUtf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::Utf8(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Utf8(error) => Some(error),
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
