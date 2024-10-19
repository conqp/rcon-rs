use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use udp_stream::UdpStream;

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

    pub async fn read_from(src: &mut UdpStream) -> std::io::Result<impl FnOnce(Header) -> Self> {
        let mut buffer = Vec::new();
        src.read_to_end(&mut buffer).await?;
        Ok(move |header| Self::new(header, buffer, payload.into()))
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
