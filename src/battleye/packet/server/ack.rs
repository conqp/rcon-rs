use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;

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

impl IntoBytes for Ack {
    fn into_bytes(self) -> impl AsRef<[u8]> {
        let [hdr0, hdr1, hdr2, hdr3, hdr4, hdr5, hdr6, hdr7] = self.header.into();
        [hdr0, hdr1, hdr2, hdr3, hdr4, hdr5, hdr6, hdr7, self.seq]
    }
}
