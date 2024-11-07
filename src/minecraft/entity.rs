use std::borrow::Cow;
use std::convert::Infallible;
use std::str::FromStr;

use crate::minecraft::serialize::Serialize;

use uuid::Uuid;

/// Identifies an entity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Entity<T> {
    /// Identified by player name.
    PlayerName(Cow<'static, str>),
    /// Identified by target selector.
    Target(T),
    /// Identified by UUID.
    Uuid(Uuid),
}

impl<T> From<Cow<'static, str>> for Entity<T> {
    fn from(cow: Cow<'static, str>) -> Self {
        Self::PlayerName(cow)
    }
}

impl<T> From<&'static str> for Entity<T> {
    fn from(s: &'static str) -> Self {
        Self::PlayerName(Cow::Borrowed(s))
    }
}

impl<T> From<String> for Entity<T> {
    fn from(string: String) -> Self {
        Self::PlayerName(Cow::Owned(string))
    }
}

impl<T> From<Uuid> for Entity<T> {
    fn from(uuid: Uuid) -> Self {
        Self::Uuid(uuid)
    }
}

impl<T> FromStr for Entity<T> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(uuid) = Uuid::from_str(s) {
            Self::Uuid(uuid)
        } else {
            Self::PlayerName(Cow::Owned(s.to_string()))
        })
    }
}

impl<T> Serialize for Entity<T>
where
    T: Serialize,
{
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::PlayerName(name) => name,
            Self::Target(target) => target.serialize(),
            Self::Uuid(uuid) => uuid.serialize(),
        }
    }
}
