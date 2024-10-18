use std::str::FromStr;

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

impl FromStr for Quirks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(&s.to_uppercase()).ok_or_else(|| format!("Invalid quirk: {s}"))
    }
}
