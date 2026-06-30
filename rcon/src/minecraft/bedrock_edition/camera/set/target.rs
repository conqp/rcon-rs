use std::borrow::Cow;

use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{Entity, Serialize};

/// Facing target.
#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    /// Facing an entity.
    Entity(Entity<TargetSelector>),
    /// Facing a position. Parameters in order are `x`, `y` and `z`.
    Position(f32, f32, f32),
}

impl Serialize for Target {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::Entity(entity) => entity.serialize(),
            Self::Position(x, y, z) => format!("{x} {y} {z}").into(),
        }
    }
}
