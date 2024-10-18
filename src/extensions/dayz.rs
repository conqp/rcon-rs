//! Extension of the `BattlEye Rcon` client for `DayZ` server.

use std::borrow::Cow;

use crate::extensions::traits::{Ban, Kick};
use crate::{battleye, Broadcast, RCon, Say};

const BROADCAST_TARGET: &str = "-1";

/// Extended `BattlEye Rcon` client for `DayZ` servers.
#[derive(Debug)]
pub struct Client {
    inner: battleye::Client,
}

impl Say for Client {
    fn say(&mut self, target: Cow<'_, str>, message: Cow<'_, str>) -> std::io::Result<()> {
        self.run(&[target, message]).map(drop)
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

impl RCon for Client {
    fn login(&mut self, password: Cow<'_, str>) -> std::io::Result<bool> {
        self.inner.login(password)
    }

    fn run(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<Vec<u8>> {
        self.inner.run(args)
    }
}
