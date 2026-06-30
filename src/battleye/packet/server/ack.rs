use super::TYPE;
use crate::battleye::header::Header;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ack {
    header: Header,
    seq: u8,
}

impl Ack {
    #[must_use]
    pub const fn new(seq: u8) -> Self {
        Self {
            header: Header::create(TYPE, &seq.to_le_bytes()),
            seq,
        }
    }
}

impl From<Ack> for [u8; 9] {
    fn from(ack: Ack) -> Self {
        let [hdr0, hdr1, hdr2, hdr3, hdr4, hdr5, hdr6, hdr7] = ack.header.into();
        [hdr0, hdr1, hdr2, hdr3, hdr4, hdr5, hdr6, hdr7, ack.seq]
    }
}
