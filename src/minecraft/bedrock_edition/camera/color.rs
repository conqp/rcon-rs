use std::borrow::Cow;

use crate::minecraft::Serialize;

/// RGB color, containing the red, green and blue values.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Create a new RGB color.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }

    /// Return the red value.
    #[must_use]
    pub const fn red(self) -> u8 {
        self.0
    }

    /// Return the green value.
    #[must_use]
    pub const fn green(self) -> u8 {
        self.1
    }

    /// Return the blue value.
    #[must_use]
    pub const fn blue(self) -> u8 {
        self.2
    }
}

impl Serialize for Color {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(format!("color {} {} {}", self.0, self.1, self.2))
    }
}
