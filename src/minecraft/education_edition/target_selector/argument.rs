use std::borrow::Cow;
use std::collections::HashMap;

use crate::minecraft::GameMode;
use crate::minecraft::Negate;
use crate::minecraft::Range;
use crate::minecraft::Serialize;

/// A target selector argument.
///
/// See the [Minecraft wiki](https://minecraft.fandom.com/wiki/Target_selectors) for details.
#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    X(f64),
    Y(f64),
    Z(f64),
    MaxRange(f64),
    MinRange(f64),
    Dx(f64),
    Dy(f64),
    Dz(f64),
    Scores(HashMap<String, Negate<Range<u64>>>),
    Tag(Negate<Option<String>>),
    Limit(i64),
    Level(Range<u64>),
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
            Self::MaxRange(max_range) => format!("r={max_range}").into(),
            Self::MinRange(min_range) => format!("rm={min_range}").into(),
            Self::Dx(dx) => format!("dx={dx}").into(),
            Self::Dy(dy) => format!("dy={dy}").into(),
            Self::Dz(dz) => format!("dz={dz}").into(),
            Self::Scores(scores) => format!("scores={}", scores.serialize()).into(),
            Self::Tag(negate) => format!("tag={}", negate.serialize()).into(),
            Self::Limit(limit) => format!("limit={limit}").into(),
            Self::Level(range) => format!("level={}", range.serialize()).into(),
            Self::GameMode(game_mode) => format!("gamemode={}", game_mode.serialize()).into(),
        }
    }
}
