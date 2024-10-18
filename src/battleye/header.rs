use std::io::ErrorKind;

use crc::{Crc, CRC_32_ISO_HDLC};
use log::{debug, error};

const INFIX: u8 = 0xFF;
const PREFIX: &[u8; 2] = b"BE";
const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

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

    pub fn read_from<T>(src: T) -> std::io::Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let buffer: [u8; Self::SIZE] = src
            .take(Self::SIZE)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| std::io::Error::from(ErrorKind::UnexpectedEof))?;
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
        let crc = self.crc32(payload);

        if crc != self.crc32 {
            error!("CRC mismatch");
            debug!("Expected: {:#010X}, but got {:#010X}", self.crc32, crc);
            return false;
        }

        true
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

#[cfg(test)]
mod tests {
    use super::crc32;

    #[test]
    fn test_crc32() {
        let checksum = crc32(0x00, 0xff, b"password");
        assert_eq!(checksum, 0x522d_26de);
    }

    #[cfg(any())]
    #[test]
    fn test_payload_checksum() {
        let payload = &[
            0x42, 0x45, 0xEE, 0x1E, 0x1B, 0xCD, 0xFF, 0x02, 0x00, 0x52, 0x43, 0x6F, 0x6E, 0x20,
            0x61, 0x64, 0x6D, 0x69, 0x6E, 0x20, 0x23, 0x30, 0x20, 0x28, 0x31, 0x32, 0x37, 0x2E,
            0x30, 0x2E, 0x30, 0x2E, 0x31, 0x3A, 0x33, 0x36, 0x31, 0x34, 0x32, 0x29, 0x20, 0x6C,
            0x6F, 0x67, 0x67, 0x65, 0x64, 0x20, 0x69, 0x6E,
        ];
        let header = Header {
            prefix: [66, 69],
            crc32: 0x58c2_dcbe,
            infix: 255,
            typ: 1,
        };
        assert!(header.is_valid(payload));
    }
}
