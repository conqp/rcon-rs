use std::borrow::Cow;

use crate::minecraft::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Ability {
    /// Gives the selector the ability to become a world builder.
    WorldBuilder,
    /// Lets the selector fly.
    MayFly,
    /// Mutes the selector.
    ///
    /// If they chat, no one can hear (or see) them.
    Mute,
}

impl Serialize for Ability {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::WorldBuilder => Cow::Borrowed("worldbuilder"),
            Self::MayFly => Cow::Borrowed("mayfly"),
            Self::Mute => Cow::Borrowed("mute"),
        }
    }
}
