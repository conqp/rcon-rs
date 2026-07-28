//! Common error types.

use std::string::FromUtf8Error;

use thiserror::Error;

/// Represents either an I/O error or a UTF-8 error.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A UTF-8 parsing error occurred.
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
}
