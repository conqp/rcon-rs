use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::Arc;

use log::debug;

use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    seq: u8,
    payload: Arc<[u8]>,
}

impl Response {
    #[must_use]
    pub const fn new(header: Header, seq: u8, payload: Arc<[u8]>) -> Self {
        Self {
            header,
            seq,
            payload,
        }
    }

    pub fn read_from(src: &UdpSocket) -> std::io::Result<impl FnOnce(Header) -> Self> {
        let mut buffer = [0; 1];

        if src.recv(&mut buffer)? < buffer.len() {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        let mut payload = Vec::new();
        let bytes = src.recv(&mut payload)?;
        debug!("Read {bytes} bytes.");

        Ok(move |header| Self::new(header, u8::from_le_bytes(buffer), payload.into()))
    }

    #[must_use]
    pub const fn seq(&self) -> u8 {
        self.seq
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl FromServer for Response {
    fn is_valid(&self) -> bool {
        self.header.is_valid(
            self.seq
                .to_le_bytes()
                .into_iter()
                .chain(self.payload.iter().copied())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}
