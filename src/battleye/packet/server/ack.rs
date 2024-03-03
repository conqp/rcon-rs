use super::TYPE;
use crate::battleye::to_server::ToServer;
use std::array::IntoIter;
use std::iter::Chain;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ack {
    seq: u8,
}

impl Ack {
    #[must_use]
    pub const fn new(seq: u8) -> Self {
        Self { seq }
    }
}

impl IntoIterator for &Ack {
    type Item = u8;
    type IntoIter = Chain<IntoIter<u8, 1>, IntoIter<u8, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        TYPE.to_le_bytes().into_iter().chain(self.seq.to_le_bytes())
    }
}

impl ToServer for &Ack {}
