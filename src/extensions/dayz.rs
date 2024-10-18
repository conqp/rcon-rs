//! Extension of the `BattlEye Rcon` client for `DayZ` server.

use crate::extensions::traits::{Ban, Kick};
use crate::{battleye, Broadcast, Players, RCon, Say};
use log::warn;
use player::Player;
use std::borrow::Cow;
use std::io::ErrorKind;
use std::str::FromStr;

mod player;

const BROADCAST_TARGET: &str = "-1";

/// Extended `BattlEye Rcon` client for `DayZ` servers.
#[derive(Debug)]
pub struct Client {
    inner: battleye::Client,
}

impl From<battleye::Client> for Client {
    fn from(client: battleye::Client) -> Self {
        Self { inner: client }
    }
}

impl Say for Client {
    fn say(&mut self, target: Cow<'_, str>, message: Cow<'_, str>) -> std::io::Result<()> {
        self.run(&["say".into(), target, message]).map(drop)
    }
}

impl Kick for Client {
    fn kick(&mut self, player: Cow<'_, str>, reason: Option<Cow<'_, str>>) -> std::io::Result<()> {
        if let Some(reason) = reason {
            self.run(&["kick".into(), player, reason])
        } else {
            self.run(&["kick".into(), player])
        }
        .map(drop)
    }
}

impl Ban for Client {
    fn ban(&mut self, player: Cow<'_, str>, reason: Option<Cow<'_, str>>) -> std::io::Result<()> {
        if let Some(reason) = reason {
            self.run(&["ban".into(), player, reason])
        } else {
            self.run(&["ban".into(), player])
        }
        .map(drop)
    }
}

impl Broadcast for Client {
    fn broadcast(&mut self, message: Cow<'_, str>) -> std::io::Result<()> {
        self.say(BROADCAST_TARGET.into(), message)
    }
}

impl Players<Player> for Client {
    fn players(&mut self) -> std::io::Result<Vec<Player>> {
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

impl RCon for Client {
    fn login(&mut self, password: Cow<'_, str>) -> std::io::Result<bool> {
        self.inner.login(password)
    }

    fn run(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<Vec<u8>> {
        self.inner.run(args)
    }
}
