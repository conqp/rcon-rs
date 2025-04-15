use std::borrow::Cow;

use crate::minecraft::Serialize;

/// Entry type for filtering.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EntryType {
    /// Only list banned IP addresses.
    Ips,
    /// Only list banned players.
    Players,
}

impl Serialize for EntryType {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::Ips => Cow::Borrowed("ips"),
            Self::Players => Cow::Borrowed("players"),
        }
    }
}
