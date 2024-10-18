use std::io::ErrorKind;
use std::net::UdpSocket;

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

    pub fn read_from(src: &UdpSocket) -> std::io::Result<impl FnOnce(Header) -> Self> {
        let mut buffer = [0; 1];

        if src.recv(&mut buffer)? < buffer.len() {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        Ok(move |header| Self::new(header, u8::from_le_bytes(buffer) != 0))
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
