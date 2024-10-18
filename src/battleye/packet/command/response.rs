use std::io::ErrorKind;
use std::sync::Arc;

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

    pub fn read_from<T>(mut src: T) -> std::io::Result<impl FnOnce(Header) -> Self>
    where
        T: Iterator<Item = u8>,
    {
        let seq = src
            .next()
            .ok_or_else(|| std::io::Error::from(ErrorKind::UnexpectedEof))?;
        let payload: Vec<u8> = src.collect();
        Ok(move |header| Self::new(header, seq, payload.into()))
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
