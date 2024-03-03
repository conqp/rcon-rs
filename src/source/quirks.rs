use crate::source::packet::Packet;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Quirk {
    Palworld = 0b000_00001,
}

impl Quirk {
    #[must_use]
    pub const fn matches(self, quirks: u8) -> bool {
        (self as u8) & quirks != 0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Quirks(pub(crate) u8);

impl Quirks {
    #[must_use]
    pub fn new(mask: u8) -> Self {
        Self(mask)
    }

    #[must_use]
    pub fn packet_is_valid(&self, packet: &Packet, id: i32) -> bool {
        if Quirk::Palworld.matches(self.0) {
            return true;
        }

        packet.id == id
    }
}

impl Default for Quirks {
    fn default() -> Self {
        Self::new(0)
    }
}
