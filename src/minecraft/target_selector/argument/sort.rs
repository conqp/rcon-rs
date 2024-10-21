use std::fmt::{Display, Formatter};

/// Sorting strategy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sort {
    /// Sort by increasing distance.
    Nearest,
    /// Sort by decreasing distance.
    Furthest,
    /// Sort randomly.
    ///
    /// Default for @r.
    Random,
    /// Do not sort.
    ///
    /// This will often return the oldest entities first due to how the game stores
    /// entities internally, but no order is guaranteed.
    ///
    /// Default for @e, @a.
    Arbitrary,
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nearest => write!(f, "nearest"),
            Self::Furthest => write!(f, "furthest"),
            Self::Random => write!(f, "random"),
            Self::Arbitrary => write!(f, "arbitrary"),
        }
    }
}
