use std::io::ErrorKind;
use std::sync::Arc;

use log::{debug, trace};

use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use crate::UdpSocketWrapper;

const MAX_PACKET_SIZE: usize = 2048;

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

    pub fn read_from(src: &UdpSocketWrapper) -> std::io::Result<impl FnOnce(Header) -> Self> {
        let mut buffer = vec![0; MAX_PACKET_SIZE];
        let size = src.recv(&mut buffer)?;

        if size < 1 {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        #[allow(unsafe_code)]
        // SAFETY: We just read this amount of bytes, which cannot exceed the initial buffer's size.
        unsafe {
            buffer.set_len(size);
        };
        debug!("Read {size} bytes.");
        trace!("Buffer: {:#04X?}", buffer);
        Ok(move |header| Self::new(header, buffer[0], buffer[1..].into()))
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
