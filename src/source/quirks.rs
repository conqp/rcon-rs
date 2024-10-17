use bitflags::bitflags;

/// Quirks for Source RCON.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Quirks(u8);

bitflags! {
    impl Quirks: u8 {
        /// Quirk for Palworld servers.
        const PALWORLD = 0b0000_0001;
    }
}
