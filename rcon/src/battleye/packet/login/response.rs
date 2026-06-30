use std::io::{Error, ErrorKind};

use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    success: bool,
}

impl Response {
    #[must_use]
    pub const fn new(header: Header, success: bool) -> Self {
        Self { header, success }
    }

    pub fn read_from<T>(mut src: T) -> std::io::Result<impl FnOnce(Header) -> Self>
    where
        T: Iterator<Item = u8>,
    {
        let success = src.next().ok_or_else(|| {
            Error::new(
                ErrorKind::UnexpectedEof,
                "Too few bytes to construct login response",
            )
        })?;
        Ok(move |header| Self::new(header, success != 0))
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }
}

impl FromServer for Response {
    fn is_valid(&self) -> bool {
        self.header.is_valid(&u8::from(self.success).to_le_bytes())
    }
}
