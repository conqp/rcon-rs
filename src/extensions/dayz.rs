//! Extension of the `BattlEye Rcon` client for `DayZ` server.

use crate::battleye::Client;
use crate::extensions::traits::{Ban, Kick};
use crate::{Broadcast, Players, RCon, Say};
use log::warn;
use player::Player;
use std::borrow::Cow;
use std::io::ErrorKind;
use std::str::FromStr;

mod player;

const BROADCAST_TARGET: &str = "-1";

/// Extended `BattlEye Rcon` client for `DayZ` servers.
pub trait DayZ {}

impl DayZ for Client {}

impl Say for Client
where
    Self: DayZ,
{
    fn say(&mut self, target: Cow<'_, str>, message: Cow<'_, str>) -> std::io::Result<()> {
        self.run(&["say".into(), target, message]).map(drop)
    }
}

impl Kick for Client
where
    Self: DayZ,
{
    fn kick(&mut self, player: Cow<'_, str>, reason: Option<Cow<'_, str>>) -> std::io::Result<()> {
        if let Some(reason) = reason {
            self.run(&["kick".into(), player, reason])
        } else {
            self.run(&["kick".into(), player])
        }
        .map(drop)
    }
}

impl Ban for Client
where
    Self: DayZ,
{
    fn ban(&mut self, player: Cow<'_, str>, reason: Option<Cow<'_, str>>) -> std::io::Result<()> {
        if let Some(reason) = reason {
            self.run(&["ban".into(), player, reason])
        } else {
            self.run(&["ban".into(), player])
        }
        .map(drop)
    }
}

impl Broadcast for Client
where
    Self: DayZ,
{
    fn broadcast(&mut self, message: Cow<'_, str>) -> std::io::Result<()> {
        self.say(BROADCAST_TARGET.into(), message)
    }
}

impl Players for Client
where
    Self: DayZ,
{
    type Player = Player;

    fn players(&mut self) -> std::io::Result<Vec<Self::Player>> {
        let result = self.run(&["players".into()])?;
        let text = String::from_utf8(result).map_err(|_| {
            std::io::Error::new(ErrorKind::InvalidData, "Response is not valid UTF-8")
        })?;

        let players: Vec<Player> = text
            .lines()
            // Discard header.
            .skip_while(|line| !line.starts_with('-'))
            .skip(1)
            // Take until footer.
            .take_while(|line| !line.starts_with('('))
            .map(Player::from_str)
            .filter_map(|result| {
                result
                    .inspect_err(|error| warn!("Failed to parse player data: {error}"))
                    .ok()
            })
            .collect();
        Ok(players)
    }
}
