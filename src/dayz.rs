//! `BattlEye RCon` extensions for `DayZ` servers.

use std::borrow::Cow;
use std::future::Future;
use std::str::FromStr;
use std::time::Duration;

use log::warn;

use crate::battleye::BattlEye;
use crate::RCon;

pub use banning::Error;
pub use banning::{BanListEntry, Target, SECS_PER_MINUTE};
pub use player::Player;

mod banning;
mod player;

const BROADCAST_TARGET: i64 = -1;
const INVALID_BAN_FORMAT_MESSAGE: &str = "Invalid ban format";

/// Extension trait for `BattlEye Rcon` clients for `DayZ` servers.
pub trait DayZ: RCon + BattlEye {
    /// Send a message to a player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn say(
        &mut self,
        index: u64,
        message: Cow<'_, str>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Broadcast a message to all players on the server.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn broadcast(
        &mut self,
        message: Cow<'_, str>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Kick a player from the server.
    ///
    /// You may specify an optional reason for the kick to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if kicking the player fails.
    fn kick(
        &mut self,
        index: u64,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Ban a player from the server.
    ///
    /// You may specify an optional reason for the ban to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if banning  the player fails.
    fn ban(
        &mut self,
        index: u64,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Returns an iterator over the server's current ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if querying the ban list fails.
    fn bans(&mut self) -> impl Future<Output = Result<Vec<BanListEntry>, crate::Error>> + Send;

    /// Add an entry to the ban list.
    ///
    /// This can be either an IP address or a UUID.
    ///
    /// You may specify an optional duration and reason for the ban to add to the ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if banning  the player fails.
    fn add_ban(
        &mut self,
        target: Target,
        duration: Option<Duration>,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Remove a player ban entry from the server's ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if unbanning  the player fails.
    fn remove_ban(&mut self, index: u64) -> impl Future<Output = std::io::Result<()>> + Send;

    /// List players on the server.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if listing the players fails.
    fn players(&mut self) -> impl Future<Output = Result<Vec<Player>, crate::Error>> + Send;

    /// Lock the server.
    ///
    /// This prevents any further clients from joining.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O error occurred.
    fn lock(&mut self) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Unlock the server.
    ///
    /// This enables other clients to join again.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O error occurred.
    fn unlock(&mut self) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Shutdown the server immediately.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O error occurred.
    fn shutdown(&mut self) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Reload server config file loaded by -config option.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O error occurred.
    fn reload(&mut self) -> impl Future<Output = std::io::Result<()>> + Send;
}

impl<T> DayZ for T
where
    T: RCon + BattlEye + Send,
{
    async fn say(&mut self, index: u64, message: Cow<'_, str>) -> std::io::Result<()> {
        self.run(&["say".into(), index.to_string().into(), message])
            .await
            .map(drop)
    }

    async fn broadcast(&mut self, message: Cow<'_, str>) -> std::io::Result<()> {
        self.run(&["say".into(), BROADCAST_TARGET.to_string().into(), message])
            .await
            .map(drop)
    }

    async fn kick(&mut self, index: u64, reason: Option<Cow<'_, str>>) -> std::io::Result<()> {
        let mut args = vec!["kick".into(), index.to_string().into()];

        if let Some(reason) = reason {
            args.push(reason);
        }

        self.run(&args).await.map(drop)
    }

    async fn ban(&mut self, index: u64, reason: Option<Cow<'_, str>>) -> std::io::Result<()> {
        let mut args = vec!["ban".into(), index.to_string().into()];

        if let Some(reason) = reason {
            args.push(reason);
        }

        self.run(&args).await.map(drop)
    }

    async fn bans(&mut self) -> Result<Vec<BanListEntry>, crate::Error> {
        self.run_utf8(&["bans"]).await.map(|text| {
            text.lines()
                .filter(|line| line.chars().next().map_or(false, char::is_numeric))
                .filter_map(|line| {
                    line.parse()
                        .inspect_err(|error| warn!(r#"Invalid ban list entry "{line}": {error}"#))
                        .ok()
                })
                .collect()
        })
    }

    async fn add_ban(
        &mut self,
        target: Target,
        duration: Option<Duration>,
        reason: Option<Cow<'_, str>>,
    ) -> Result<(), Error> {
        let mut args: Vec<Cow<'_, str>> = vec!["addBan".into()];

        match target {
            Target::Ip(ip) => args.push(ip.to_string().into()),
            Target::Uuid(uuid) => args.push(uuid.to_string().replace('-', "").into()),
        }

        args.push(
            duration
                .map_or(0, |duration| duration.as_secs() / SECS_PER_MINUTE)
                .to_string()
                .into(),
        );

        // FIXME: The appended reason currently does not appear in the ban list.
        // TODO: Investigate this.
        if let Some(reason) = reason {
            args.push(reason);
        }

        let response = self.run(&args).await?;

        if response == INVALID_BAN_FORMAT_MESSAGE.as_bytes() {
            Err(Error::InvalidBanFormat)
        } else {
            Ok(())
        }
    }

    async fn remove_ban(&mut self, id: u64) -> std::io::Result<()> {
        self.run(&["removeBan", &id.to_string()]).await.map(drop)
    }

    async fn players(&mut self) -> Result<Vec<Player>, crate::Error> {
        self.run_utf8(&["players"]).await.map(|text| {
            text.lines()
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
                .collect()
        })
    }

    async fn lock(&mut self) -> std::io::Result<()> {
        self.run(&["#lock"]).await.map(drop)
    }

    async fn unlock(&mut self) -> std::io::Result<()> {
        self.run(&["#unlock"]).await.map(drop)
    }

    async fn shutdown(&mut self) -> std::io::Result<()> {
        self.run(&["#shutdown"]).await.map(drop)
    }

    async fn reload(&mut self) -> std::io::Result<()> {
        self.run(&["#init"]).await.map(drop)
    }
}
