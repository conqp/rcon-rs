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
    fn serialize(&self) -> Cow<'_, str> {
        match self {
            GameMode::Survival => Cow::Borrowed("survival"),
            GameMode::Creative => Cow::Borrowed("creative"),
            GameMode::Adventure => Cow::Borrowed("adventure"),
            GameMode::Spectator => Cow::Borrowed("spectator"),
        }
    }
}
