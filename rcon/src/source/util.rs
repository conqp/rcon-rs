use std::io::{Error, ErrorKind};

/// Converts any fitting error type into an `InvalidData` error.
pub fn invalid_data<T>(reason: T) -> Error
where
    T: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Error::new(ErrorKind::InvalidData, reason)
}
