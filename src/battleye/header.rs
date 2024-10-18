use std::io::ErrorKind;
use std::net::UdpSocket;

use crc::{Crc, CRC_32_CKSUM};

const INFIX: u8 = 0xFF;
const PREFIX: &[u8; 2] = b"BE";
const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    prefix: [u8; 2],
    crc32: u32,
    infix: u8,
    typ: u8,
}

impl Header {
    pub const SIZE: usize = 8;

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

    pub fn read_from(src: &UdpSocket) -> std::io::Result<Self> {
        let mut buffer = [0; 8];

        if src.recv(&mut buffer)? < buffer.len() {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        Ok(Self::from(buffer))
    }

    #[must_use]
    pub const fn typ(self) -> u8 {
        self.typ
    }

    pub fn crc32(self, payload: &[u8]) -> u32 {
        crc32(self.typ, self.infix, payload)
    }

    pub fn is_valid(self, payload: &[u8]) -> bool {
        self.crc32(payload) == self.crc32
    }
}

impl From<[u8; Self::SIZE]> for Header {
    fn from(buffer: [u8; Self::SIZE]) -> Self {
        Self::new(
            [buffer[0], buffer[1]],
            u32::from_le_bytes([buffer[2], buffer[3], buffer[4], buffer[5]]),
            buffer[6],
            buffer[7],
        )
    }
}

impl From<Header> for [u8; Header::SIZE] {
    fn from(header: Header) -> [u8; Header::SIZE] {
        let [prefix0, prefix1] = header.prefix;
        let [a, b, c, d] = header.crc32.to_le_bytes();
        [prefix0, prefix1, a, b, c, d, header.infix, header.typ]
    }
}

fn crc32(typ: u8, infix: u8, payload: &[u8]) -> u32 {
    let mut crc = CRC32.digest();
    crc.update(&[infix, typ]);
    crc.update(payload);
    crc.finalize()
}
