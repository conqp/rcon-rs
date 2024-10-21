use std::borrow::Cow;
use std::ops::Not;

use crate::minecraft::serialize::Serialize;

/// A value that may be either included or excluded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Negate<T> {
    /// Value is included.
    Include(T),
    /// Value is excluded.
    Exclude(T),
}

impl<T> From<T> for Negate<T> {
    fn from(value: T) -> Self {
        Self::Include(value)
    }
}

impl<T> Not for Negate<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Include(value) => Self::Exclude(value),
            Self::Exclude(value) => Self::Include(value),
        }
    }
}

impl<T> Serialize for Negate<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Cow<'_, str> {
        match self {
            Self::Include(value) => value.serialize(),
            Self::Exclude(value) => format!("!{}", value.serialize()).into(),
        }
    }
}
