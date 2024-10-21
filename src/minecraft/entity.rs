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

impl<T> From<String> for Entity<T> {
    fn from(s: String) -> Self {
        Self::PlayerName(s)
    }
}

impl<T> From<Uuid> for Entity<T> {
    fn from(uuid: Uuid) -> Self {
        Self::Uuid(uuid)
    }
}

impl<T> Serialize for Entity<T>
where
    T: Serialize,
{
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::PlayerName(name) => name.serialize(),
            Self::Target(target) => target.serialize(),
            Self::Uuid(uuid) => uuid.serialize(),
        }
    }
}
