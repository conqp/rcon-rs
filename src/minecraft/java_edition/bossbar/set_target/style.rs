use std::fmt::{self, Display};

/// Available bossabar styles.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Style {
    /// 6 segments
    Notched6,
    /// 10 segments
    Notched10,
    /// 12 segments
    Notched12,
    /// 20 segments
    Notched20,
    /// Contiguous
    Progress,
}

impl Style {
    /// Returns a `str` representation of the enum value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notched6 => "notched_6",
            Self::Notched10 => "notched_10",
            Self::Notched12 => "notched_12",
            Self::Notched20 => "notched_20",
            Self::Progress => "progress",
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
