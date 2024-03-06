use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use log::debug;
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
        let mut buffer = [0; 1];
        src.read_exact(&mut buffer).await?;
        let mut payload = Vec::new();
        let bytes = src.read_to_end(&mut payload).await?;
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
