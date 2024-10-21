use std::borrow::Cow;

use crate::minecraft::{Entity, Serialize};

pub use argument::Argument;

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
    /// Selects the player's agent only.
    #[cfg(feature = "minecraft-education-edition")]
    PlayerAgent(Option<Vec<Argument>>),
    /// Selects all agents.
    ///
    /// Works only if more than one agent exists.
    #[cfg(feature = "minecraft-education-edition")]
    AllAgents(Option<Vec<Argument>>),
    /// Selects the player who interacts with a button in a JSON NPC dialogue.
    #[cfg(feature = "minecraft-bedrock-edition")]
    Initiator(Option<Vec<Argument>>),
}

impl From<TargetSelector> for Entity<TargetSelector> {
    fn from(selector: TargetSelector) -> Self {
        Self::Target(selector)
    }
}

impl Serialize for TargetSelector {
    fn serialize(&self) -> Cow<'_, str> {
        match self {
            Self::NearestPlayer(arguments) => format!("@p{}", arguments.serialize()).into(),
            Self::RandomPlayer(arguments) => format!("@r{}", arguments.serialize()).into(),
            Self::EveryPlayer(arguments) => format!("@a{}", arguments.serialize()).into(),
            Self::AliveEntities(arguments) => format!("@e{}", arguments.serialize()).into(),
            Self::Executor(arguments) => format!("@s{}", arguments.serialize()).into(),
            #[cfg(feature = "minecraft-education-edition")]
            Self::PlayerAgent(arguments) => format!("@c{}", arguments.serialize()).into(),
            #[cfg(feature = "minecraft-education-edition")]
            Self::AllAgents(arguments) => format!("@v{}", arguments.serialize()).into(),
            #[cfg(feature = "minecraft-bedrock-edition")]
            Self::Initiator(arguments) => format!("@initiator{}", arguments.serialize()).into(),
        }
    }
}