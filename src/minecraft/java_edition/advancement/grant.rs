use std::borrow::Cow;

use crate::minecraft::ResourceLocation;
use crate::minecraft::Serialize;

/// An advancement grant.
///
/// See the [Minecraft wik](https://minecraft.fandom.com/wiki/Commands/advancement) for details.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Grant {
    /// Adds or removes all loaded advancements.
    Everything,
    /// Adds or removes a single advancement or criterion.
    Only {
        /// Specifies a valid resource location of the advancement to target.
        advancement: ResourceLocation,
        /// Specifies a valid criterion of the advancement to manipulate.
        criterion: Option<String>,
    },
    /// Adds or removes an advancement and all its child advancements.
    From(ResourceLocation),
    /// Specifies an advancement, and adds or removes all its parent advancements,
    /// and all its child advancements.
    Through(ResourceLocation),
    /// Adds or removes an advancement and all its parent advancements
    /// until the root for addition/removal.
    Until(ResourceLocation),
}

impl Serialize for Grant {
    fn serialize(&self) -> Cow<'_, str> {
        match self {
            Self::Everything => Cow::Borrowed("everything"),
            Self::Only {
                advancement,
                criterion,
            } => criterion
                .as_ref()
                .map_or_else(
                    || format!("only {}", advancement.serialize()),
                    |criterion| {
                        format!("only {} {}", advancement.serialize(), criterion.serialize())
                    },
                )
                .into(),
            Self::From(advancement) => format!("from {}", advancement.serialize()).into(),
            Self::Through(advancement) => format!("through {}", advancement.serialize()).into(),
            Self::Until(advancement) => format!("until {}", advancement.serialize()).into(),
        }
    }
}
