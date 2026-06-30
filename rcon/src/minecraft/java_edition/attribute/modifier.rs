use std::borrow::Cow;

use crate::minecraft::Serialize;

/// Modifier type.
pub enum Modifier {
    /// Add offset.
    Add,
    /// Multiplier
    Multiply,
    /// Multiply with base value.
    MultiplyBase,
}

impl Serialize for Modifier {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::Add => Cow::Borrowed("add"),
            Self::Multiply => Cow::Borrowed("multiply"),
            Self::MultiplyBase => Cow::Borrowed("multiply_base"),
        }
    }
}
