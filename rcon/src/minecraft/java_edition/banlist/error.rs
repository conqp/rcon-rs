use std::string::FromUtf8Error;

use thiserror::Error;

/// Errors that can occur when querying the banlist.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A UTF-8 error occurred.
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    /// An invalid ban list entry was encountered.
    #[error("Invalid entry: {0}")]
    InvalidEntry(String),
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::Io(error) => Self::Io(error),
            crate::Error::Utf8(error) => Self::Utf8(error),
        }
    }
}
