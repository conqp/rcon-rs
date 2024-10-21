use std::borrow::Cow;
use std::ops::{RangeFrom, RangeInclusive, RangeToInclusive};

use crate::minecraft::serialize::Serialize;

/// A range of non-negative floats.
///
/// See the [Minecraft wiki](https://minecraft.fandom.com/wiki/Argument_types#minecraft:float_range)
/// for details.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Range<T> {
    /// An exact value of `T`.
    Exact(T),
    /// A range starting at some value without upper bound.
    From(RangeFrom<T>),
    /// A range without a lower bound up to an inclusive end value.
    ToInclusive(RangeToInclusive<T>),
    /// A range from a start value up to an inclusive end value.
    Inclusive(RangeInclusive<T>),
}

impl<T> Serialize for Range<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Cow<'_, str> {
        match self {
            Self::Exact(value) => value.serialize(),
            Self::From(range) => format!("{}..", range.start.serialize()).into(),
            Self::ToInclusive(range) => format!("..{}", range.end.serialize()).into(),
            Self::Inclusive(range) => {
                format!("{}..{}", range.start().serialize(), range.end().serialize()).into()
            }
        }
    }
}

impl<T> From<T> for Range<T> {
    fn from(value: T) -> Self {
        Self::Exact(value)
    }
}

impl<T> From<RangeFrom<T>> for Range<T> {
    fn from(value: RangeFrom<T>) -> Self {
        Self::From(value)
    }
}

impl<T> From<RangeToInclusive<T>> for Range<T> {
    fn from(value: RangeToInclusive<T>) -> Self {
        Self::ToInclusive(value)
    }
}

impl<T> From<RangeInclusive<T>> for Range<T> {
    fn from(value: RangeInclusive<T>) -> Self {
        Self::Inclusive(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Range;
    use crate::minecraft::serialize::Serialize;

    #[test]
    fn test_range_exact() {
        let num = 0.0;
        let range = Range::Exact(num);
        assert_eq!(range.serialize(), num.to_string());

        let num = -5.4;
        let range = Range::Exact(num);
        assert_eq!(range.serialize(), num.to_string());
    }

    #[test]
    fn test_range_from() {
        let start = -100.76;
        let range = Range::From(start..);
        assert_eq!(range.serialize(), format!("{start}.."));
    }

    #[test]
    fn test_range_to_inclusive() {
        let end = 100.0;
        let range = Range::ToInclusive(..=end);
        assert_eq!(range.serialize(), format!("..{end}"));
    }

    #[test]
    fn test_range_inclusive() {
        let start = 0.0;
        let end = 5.2;
        let range = Range::Inclusive(start..=end);
        assert_eq!(range.serialize(), format!("{start}..{end}"));
    }
}
