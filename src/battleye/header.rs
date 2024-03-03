use crc::{Crc, CRC_32_CKSUM};
use std::array::IntoIter;
use std::iter::Chain;
use tokio::io::AsyncReadExt;
use udp_stream::UdpStream;

const INFIX: u8 = 0xFF;
const PREFIX: &[u8; 2] = b"BE";
const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);
pub const SIZE: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    prefix: [u8; 2],
    crc32: u32,
    infix: u8,
    typ: u8,
}

impl Header {
    #[must_use]
    pub const fn new(prefix: [u8; 2], crc32: u32, infix: u8, typ: u8) -> Self {
        Self {
            prefix,
            crc32,
            infix,
            typ,
        }
    }

    #[must_use]
    pub fn create(typ: u8, payload: &[u8]) -> Self {
        Self::new(*PREFIX, crc32(typ, INFIX, payload), INFIX, typ)
    }

    pub async fn read_from(src: &mut UdpStream) -> std::io::Result<Self> {
        let mut buffer = [0; 8];
        src.read_exact(&mut buffer).await?;
        Ok(Self::from(buffer))
    }

    #[must_use]
    pub const fn typ(&self) -> u8 {
        self.typ
    }

    pub fn crc32(&self, payload: &[u8]) -> u32 {
        crc32(self.typ, self.infix, payload)
    }

    pub fn is_valid(&self, payload: &[u8]) -> bool {
        self.crc32(payload) == self.crc32
    }
}

impl From<[u8; SIZE]> for Header {
    fn from(buffer: [u8; SIZE]) -> Self {
        Self::new(
            [buffer[0], buffer[1]],
            u32::from_le_bytes([buffer[2], buffer[3], buffer[4], buffer[5]]),
            buffer[6],
            buffer[7],
        )
    }
}

impl IntoIterator for &Header {
    type Item = u8;
    type IntoIter = Chain<
        Chain<Chain<IntoIter<Self::Item, 2>, IntoIter<Self::Item, 4>>, IntoIter<Self::Item, 1>>,
        IntoIter<Self::Item, 1>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.prefix
            .into_iter()
            .chain(self.crc32.to_le_bytes())
            .chain(self.infix.to_le_bytes())
            .chain(self.typ.to_le_bytes())
    }
}

fn crc32(typ: u8, infix: u8, payload: &[u8]) -> u32 {
    CRC32.checksum(
        infix
            .to_le_bytes()
            .into_iter()
            .chain(typ.to_le_bytes())
            .chain(payload.iter().copied())
            .collect::<Vec<_>>()
            .as_slice(),
    )
}
