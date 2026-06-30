use std::borrow::Cow;
use std::collections::BTreeMap;

pub use sort::Sort;

use crate::minecraft::{GameMode, Negate, Range, Serialize, UnsignedFloat};

mod sort;

/// A target selector argument.
///
/// See the [Minecraft wiki](https://minecraft.fandom.com/wiki/Target_selectors) for details.
#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    /// x-coordinate
    X(f64),
    /// y-coordinate
    Y(f64),
    /// z-coordinate
    Z(f64),
    /// Distance
    Distance(Range<UnsignedFloat>),
    /// X volume dimension
    Dx(f64),
    /// Y volume dimension
    Dy(f64),
    /// Z volume dimension
    Dz(f64),
    /// Score filters
    Scores(BTreeMap<String, Range<u64>>),
    /// Tag filters
    Tag(Negate<Option<String>>),
    /// Team filters
    Team(Negate<Option<String>>),
    /// Sorting
    Sort(Sort),
    /// Limiting
    Limit(u64),
    /// Level filter
    Level(Range<u64>),
    /// Game mode filter
    GameMode(Negate<GameMode>),
    // TODO implement further selector attributes
    // <https://minecraft.fandom.com/wiki/Target_selectors>
}

impl Serialize for Argument {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::X(x) => format!("x={x}").into(),
            Self::Y(y) => format!("y={y}").into(),
            Self::Z(z) => format!("z={z}").into(),
            Self::Distance(distance) => format!("distance={}", distance.serialize()).into(),
            Self::Dx(dx) => format!("dx={dx}").into(),
            Self::Dy(dy) => format!("dy={dy}").into(),
            Self::Dz(dz) => format!("dz={dz}").into(),
            Self::Scores(scores) => format!("scores={}", scores.serialize()).into(),
            Self::Tag(negate) => format!("tag={}", negate.serialize()).into(),
            Self::Team(negate) => format!("team={}", negate.serialize()).into(),
            Self::Sort(sort) => format!("sort={sort}").into(),
            Self::Limit(limit) => format!("limit={limit}").into(),
            Self::Level(range) => format!("level={}", range.serialize()).into(),
            Self::GameMode(game_mode) => format!("gamemode={}", game_mode.serialize()).into(),
        }
    }
}
