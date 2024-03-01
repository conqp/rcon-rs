use std::io::{Error, ErrorKind};

pub fn invalid_data<T>(reason: T) -> Error
where
    T: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Error::new(ErrorKind::InvalidData, reason)
}
