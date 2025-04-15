use std::borrow::Cow;

use crate::minecraft::serialize::Serialize;

/// Available game modes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameMode {
    /// Survival mode.
    Survival,
    /// Creative mode.
    Creative,
    /// Adventure mode.
    Adventure,
    /// Spectator mode.
    Spectator,
}

impl Serialize for GameMode {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::Survival => Cow::Borrowed("survival"),
            Self::Creative => Cow::Borrowed("creative"),
            Self::Adventure => Cow::Borrowed("adventure"),
            Self::Spectator => Cow::Borrowed("spectator"),
        }
    }
}
