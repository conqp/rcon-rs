use std::borrow::Cow;

use crate::minecraft::Serialize;

/// Entry type for filtering.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EntryType {
    Ips,
    Players,
}

impl Serialize for EntryType {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            EntryType::Ips => Cow::Borrowed("ips"),
            EntryType::Players => Cow::Borrowed("players"),
        }
    }
}
