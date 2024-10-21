use std::borrow::Cow;
use std::collections::HashMap;

use crate::minecraft::GameMode;
use crate::minecraft::Negate;
use crate::minecraft::Range;
use crate::minecraft::Serialize;
use crate::minecraft::UnsignedFloat;

pub use sort::Sort;

mod sort;

/// A target selector argument.
///
/// See the [Minecraft wiki](https://minecraft.fandom.com/wiki/Target_selectors) for details.
#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    X(f64),
    Y(f64),
    Z(f64),
    Distance(Range<UnsignedFloat>),
    Dx(f64),
    Dy(f64),
    Dz(f64),
    Scores(HashMap<String, Range<u64>>),
    Tag(Negate<Option<String>>),
    Team(Negate<Option<String>>),
    Sort(Sort),
    Limit(u64),
    Level(Range<u64>),
    GameMode(Negate<GameMode>),
    // TODO implement further selector attributes
    // <https://minecraft.fandom.com/wiki/Target_selectors>
}

impl Serialize for Argument {
    fn serialize(&self) -> Cow<'_, str> {
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
