use std::borrow::Cow;
use std::num::NonZero;

pub use color::Color;
pub use style::Style;

use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{Entity, Serialize};

mod color;
mod style;

/// Configuration values to set on the bossbar.
#[derive(Clone, Debug, PartialEq)]
pub enum SetTarget {
    /// Set the text color (if no color was specified as part of a text component) and bar color.
    /// Defaults to [`Color::White`] upon creation.
    Color(Color),
    /// Set the bossbar's maximum value. Defaults to 100 upon creation.
    ///
    /// Must be a 32-bit integer number. And it must be between `1` and `2147483647` (inclusive).
    Max(NonZero<i32>),
    /// Set the bossbar's name.
    // TODO: This is actually a raw JSON object called "component".
    Name(String),
    /// Change the set of players to whom the bar is visible. Defaults to none upon creation.
    ///
    /// Must be a raw JSON text.
    Players(Vec<Entity<TargetSelector>>),
    /// Set the bossbar's visual amount of segments:
    ///
    /// * continuous,
    /// * 6 segments,
    /// * 10 segments,
    /// * 12 segments or
    /// * 20 segments.
    ///
    /// Defaults to progress upon creation.
    Style(Style),
    /// Set the bossbar's current value. Defaults to 0 upon creation.
    ///
    /// Must be a 32-bit integer number. And it must be between `0` and `2147483647` (inclusive).
    Value(i32),
    /// Set the bossbar's visibility. Defaults to true upon creation.
    Visible(bool),
}

impl Serialize for SetTarget {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(match self {
            Self::Color(color) => format!("color {color}"),
            Self::Max(max) => format!("max {max}"),
            Self::Name(name) => format!("name {name}"),
            Self::Players(players) => format!("players {}", players.serialize()),
            Self::Style(style) => format!("style {style}"),
            Self::Value(value) => format!("value {value}"),
            Self::Visible(visible) => format!("visible {visible}"),
        })
    }
}
