use crate::source::packet::Packet;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Fix {
    Palworld,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Fixes(HashSet<Fix>);

impl Fixes {
    #[must_use]
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    #[must_use]
    pub const fn get(&self) -> &HashSet<Fix> {
        &self.0
    }

    pub fn add(&mut self, fix: Fix) -> bool {
        self.0.insert(fix)
    }

    pub fn remove(&mut self, fix: &Fix) -> bool {
        self.0.remove(fix)
    }

    #[must_use]
    pub fn packet_is_valid(&self, packet: &Packet, id: i32) -> bool {
        if self.0.contains(&Fix::Palworld) {
            return true;
        }

        packet.id == id
    }
}

impl Default for Fixes {
    fn default() -> Self {
        Self::new()
    }
}
