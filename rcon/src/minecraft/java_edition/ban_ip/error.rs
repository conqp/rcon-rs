use std::string::FromUtf8Error;

use thiserror::Error;

/// Errors that can occur when banning an IP.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A UTF-8 error occurred.
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    /// The specified player is unknown.
    #[error("Unknown player")]
    UnknownPlayer,
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::Io(error) => Self::Io(error),
            crate::Error::Utf8(error) => Self::Utf8(error),
        }
    }
}
