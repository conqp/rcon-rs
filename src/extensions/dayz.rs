//! Extension of the `BattlEye Rcon` client for `DayZ` server.

use std::borrow::Cow;

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
