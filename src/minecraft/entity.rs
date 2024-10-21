use std::borrow::Cow;

use crate::minecraft::serialize::Serialize;

use uuid::Uuid;

/// Identifies an entity.
#[derive(Clone, Debug, PartialEq)]
pub enum Entity<T> {
    /// Identified by player name.
    PlayerName(String),
    /// Identified by target_selector selector.
    Target(T),
    /// Identified by UUID.
    Uuid(Uuid),
}

impl<T> Serialize for Entity<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Cow<'_, str> {
        match self {
            Self::PlayerName(name) => name.serialize(),
            Self::Target(target) => target.serialize(),
            Self::Uuid(uuid) => uuid.serialize(),
        }
    }
}
