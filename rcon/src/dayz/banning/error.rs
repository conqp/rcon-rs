use std::io;

use thiserror::Error;

/// Represents either an I/O error or an invalid ban format.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// An invalid ban format was provided.
    #[error("Invalid ban format")]
    InvalidBanFormat,
}

impl From<Error> for crate::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(error) => Self::Io(error),
            Error::InvalidBanFormat => Self::Io(io::Error::other("Invalid ban format")),
        }
    }
}
