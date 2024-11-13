use std::fmt::Display;
use std::string::FromUtf8Error;

/// Errors that can occur when querying the banlist.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// A UTF-8 error occurred.
    Utf8(FromUtf8Error),
    /// An invalid ban list entry was encountered.
    InvalidEntry(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Utf8(err) => err.fmt(f),
            Self::InvalidEntry(text) => write!(f, "Invalid entry: {text}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Utf8(err) => Some(err),
            Self::InvalidEntry(_) => None,
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
            crate::Error::Io(err) => Self::Io(err),
            crate::Error::Utf8(err) => Self::Utf8(err),
        }
    }
}
