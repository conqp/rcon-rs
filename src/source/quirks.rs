use crate::source::packet::Packet;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Quirk {
    Palworld,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Quirks(pub(crate) HashSet<Quirk>);

impl Quirks {
    #[must_use]
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    #[must_use]
    pub fn packet_is_valid(&self, packet: &Packet, id: i32) -> bool {
        if self.0.contains(&Quirk::Palworld) {
            return true;
        }

        packet.id == id
    }
}

impl Default for Quirks {
    fn default() -> Self {
        Self::new()
    }
}
