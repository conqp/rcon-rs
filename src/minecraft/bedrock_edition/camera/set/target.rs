use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{Entity, Serialize};
use std::borrow::Cow;

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
            Target::Entity(entity) => entity.serialize(),
            Target::Position(x, y, z) => format!("{x} {y} {z}").into(),
        }
    }
}
