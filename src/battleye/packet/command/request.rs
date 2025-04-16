use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;

const SEQ: u8 = 0x00;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    header: Header,
    seq: u8,
    command: Vec<u8>,
}

impl Request {
    #[must_use]
    pub const fn new(header: Header, seq: u8, command: Vec<u8>) -> Self {
        Self {
            header,
            seq,
            command,
        }
    }

    #[must_use]
    pub fn keepalive() -> Self {
        Self::new(Header::create(TYPE, &SEQ.to_le_bytes()), SEQ, Vec::new())
    }
}

impl From<&[u8]> for Request {
    fn from(command: &[u8]) -> Self {
        Self::new(
            Header::create(
                TYPE,
                SEQ.to_le_bytes()
                    .into_iter()
                    .chain(command.iter().copied())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            SEQ,
            command.to_vec(),
        )
    }
}

impl IntoBytes for Request {
    fn into_bytes(self) -> impl AsRef<[u8]> {
        let header: [u8; Header::SIZE] = self.header.into();
        let command = self.command;
        let mut buffer = Vec::with_capacity(Header::SIZE + command.len());
        buffer.extend_from_slice(&header);
        buffer.push(self.seq);
        buffer.extend_from_slice(&command);
        buffer
    }
}
