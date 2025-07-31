use std::fmt::{self, Display};

/// Target types retrievable from the bossbar.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GetTarget {
    /// The bossbar's maximum value.
    Max,
    /// The set of players to whom the bar is visible.
    Players,
    /// The bossbar's current value.
    Value,
    /// The bossbar's visibility.
    Visible,
}

impl GetTarget {
    /// Returns a `str` representation of the enum value.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Max => "max",
            Self::Players => "players",
            Self::Value => "value",
            Self::Visible => "visible",
        }
    }
}

impl Display for GetTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
