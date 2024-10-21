use std::borrow::Cow;

use crate::minecraft::serialize::Serialize;
use crate::minecraft::target_selector::TargetSelector;

use uuid::Uuid;

/// Identifies an entity.
#[derive(Clone, Debug, PartialEq)]
pub enum Entity {
    /// Identified by player name.
    PlayerName(String),
    /// Identified by target_selector selector.
    Target(TargetSelector),
    /// Identified by UUID.
    Uuid(Uuid),
}

impl Serialize for Entity {
    fn serialize(&self) -> Cow<'_, str> {
        match self {
            Self::PlayerName(name) => name.serialize(),
            Self::Target(target) => target.serialize(),
            Self::Uuid(uuid) => uuid.serialize(),
        }
    }
}
