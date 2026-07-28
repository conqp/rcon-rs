use std::string::FromUtf8Error;

use thiserror::Error;

/// Errors that can occur when banning a player.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A UTF-8 error occurred.
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    /// An unknown selector has been specified.
    #[error("Unknown player")]
    UnknownSelector,
    /// Another error occurred.
    #[error("Unexpected error: {0}")]
    Other(String),
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::Io(error) => Self::Io(error),
            crate::Error::Utf8(error) => Self::Utf8(error),
        }
    }
}
