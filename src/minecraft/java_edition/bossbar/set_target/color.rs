use std::fmt::{self, Display};

/// Available bossbar colors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(missing_docs)]
pub enum Color {
    Blue,
    Green,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
}

impl Color {
    /// Returns a `str` representation of the enum value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Pink => "pink",
            Self::Purple => "purple",
            Self::Red => "red",
            Self::White => "white",
            Self::Yellow => "yellow",
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
