use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::Entity;

/// Facing target.
#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    /// Facing an entity.
    Entity(Entity<TargetSelector>),
    /// Facing a position. Parameters in order are `x`, `y` and `z`.
    Position(f32, f32, f32),
}
