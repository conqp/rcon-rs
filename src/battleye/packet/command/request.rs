use std::borrow::Cow;

use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;

const SEQ: u8 = 0x00;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    header: Header,
    seq: u8,
    command: String,
}

impl Request {
    #[must_use]
    pub const fn new(header: Header, seq: u8, command: String) -> Self {
        Self {
            header,
            seq,
            command,
        }
    }
}

impl<'cow, T> From<T> for Request
where
    T: Into<Cow<'cow, str>>,
{
    fn from(command: T) -> Self {
        let command = command.into();

        Self::new(
            Header::create(
                TYPE,
                SEQ.to_le_bytes()
                    .into_iter()
                    .chain(command.as_bytes().iter().copied())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            SEQ,
            command.into_owned(),
        )
    }
}

impl IntoBytes for Request {
    fn into_bytes(self) -> impl AsRef<[u8]> {
        let header: [u8; Header::SIZE] = self.header.into();
        let command = self.command.as_bytes();
        let mut buffer = Vec::with_capacity(Header::SIZE + command.len());
        buffer.extend_from_slice(&header);
        buffer.push(self.seq);
        buffer.extend_from_slice(command);
        buffer
    }
}
