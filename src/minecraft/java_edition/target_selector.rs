//! Target selector and associated types.

use std::borrow::Cow;

use crate::minecraft::{Entity, Serialize};

pub use argument::{Argument, Sort};

mod argument;

/// A target selector.
///
/// See the [Minecraft wiki](https://minecraft.fandom.com/wiki/Target_selectors) for details.
#[derive(Clone, Debug, PartialEq)]
pub enum TargetSelector {
    /// Selects the nearest player from the command's execution.
    ///
    /// If there are multiple nearest players,
    /// caused by them being precisely the same distance away,
    /// the player who most recently joined the server is selected.
    NearestPlayer(Option<Vec<Argument>>),
    /// Selects a random player.
    RandomPlayer(Option<Vec<Argument>>),
    /// Selects a random player.
    EveryPlayer(Option<Vec<Argument>>),
    /// Selects all alive entities (including players) in loaded chunks.
    AliveEntities(Option<Vec<Argument>>),
    /// Selects the entity (alive or not) that executed the command.
    ///
    /// It does not select anything if the command was run by a command block or server console.
    Executor(Option<Vec<Argument>>),
}

impl From<TargetSelector> for Entity<TargetSelector> {
    fn from(selector: TargetSelector) -> Self {
        Self::Target(selector)
    }
}

impl Serialize for TargetSelector {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::NearestPlayer(arguments) => format!("@p{}", arguments.serialize()).into(),
            Self::RandomPlayer(arguments) => format!("@r{}", arguments.serialize()).into(),
            Self::EveryPlayer(arguments) => format!("@a{}", arguments.serialize()).into(),
            Self::AliveEntities(arguments) => format!("@e{}", arguments.serialize()).into(),
            Self::Executor(arguments) => format!("@s{}", arguments.serialize()).into(),
        }
    }
}
