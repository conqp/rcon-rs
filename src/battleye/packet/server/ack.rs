use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::to_server::ToServer;
use std::array::IntoIter;
use std::iter::Chain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ack {
    header: Header,
    seq: u8,
}

impl Ack {
    #[must_use]
    pub fn new(seq: u8) -> Self {
        Self {
            header: Header::create(TYPE, &seq.to_le_bytes()),
            seq,
        }
    }
}

impl IntoIterator for Ack {
    type Item = u8;
    type IntoIter = Chain<<Header as IntoIterator>::IntoIter, IntoIter<u8, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.header.into_iter().chain(self.seq.to_le_bytes())
    }
}

impl ToServer for Ack {}
