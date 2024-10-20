use std::borrow::Cow;
use std::io::ErrorKind;
use std::str::FromStr;
use std::time::Duration;

use log::warn;

use crate::battleye::BattlEye;
use crate::extensions::traits::{Ban, Kick};
use crate::{BanList, Broadcast, Players, RCon, Say, Target};

use ban_list_entry::{BanListEntry, PERM_BAN, SECS_PER_MINUTE};
use player::Player;

mod ban_list_entry;
mod player;

const BROADCAST_TARGET: i64 = -1;
const INVALID_BAN_FORMAT_MESSAGE: &str = "Invalid ban format";

/// Extended `BattlEye Rcon` client for `DayZ` servers.
trait DayZ: RCon + BattlEye {}

impl<T> DayZ for T where T: RCon + BattlEye {}

impl<T> Say for T
where
    T: DayZ + Send,
{
    async fn say<P>(&mut self, player: P, message: Cow<'_, str>) -> std::io::Result<()>
    where
        P: ToString + Send,
    {
        self.run(&["say".into(), player.to_string().into(), message])
            .await
            .map(drop)
    }
}

impl<T> Kick for T
where
    T: DayZ + Send,
{
    async fn kick<P>(&mut self, player: P, reason: Option<Cow<'_, str>>) -> std::io::Result<()>
    where
        P: ToString + Send,
    {
        if let Some(reason) = reason {
            self.run(&["kick".into(), player.to_string().into(), reason])
                .await
        } else {
            self.run(&["kick".into(), player.to_string().into()]).await
        }
        .map(drop)
    }
}

impl<T> Ban for T
where
    T: DayZ + Send,
{
    async fn ban<P>(&mut self, player: P, reason: Option<Cow<'_, str>>) -> std::io::Result<()>
    where
        P: ToString + Send,
    {
        if let Some(reason) = reason {
            self.run(&["ban".into(), player.to_string().into(), reason])
                .await
        } else {
            self.run(&["ban".into(), player.to_string().into()]).await
        }
        .map(drop)
    }
}

impl<T> BanList for T
where
    T: DayZ + Send,
{
    type BanListEntry = BanListEntry;

    async fn bans(&mut self) -> std::io::Result<impl Iterator<Item = Self::BanListEntry>> {
        self.run_utf8_lossy(&["bans".into()]).await.map(|text| {
            text.lines()
                .filter(|line| line.chars().next().map_or(false, char::is_numeric))
                .filter_map(|line| {
                    BanListEntry::from_str(line)
                        .inspect_err(|error| warn!(r#"Invalid ban list entry "{line}": {error}"#))
                        .ok()
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
    }

    async fn add_ban(
        &mut self,
        target: Target,
        duration: Option<Duration>,
        reason: Option<Cow<'_, str>>,
    ) -> std::io::Result<()> {
        let mut args: Vec<Cow<'_, str>> = vec!["addBan".into()];

        match target {
            Target::Ip(ip) => args.push(ip.to_string().into()),
            Target::Uuid(uuid) => args.push(uuid.to_string().replace('-', "").into()),
        }

        if let Some(duration) = duration {
            args.push((duration.as_secs() / SECS_PER_MINUTE).to_string().into());
        } else if reason.is_some() {
            args.push(PERM_BAN.into());
        }

        // FIXME: The appended reason currently does not appear in the ban list.
        // TODO: Investigate this.
        if let Some(reason) = reason {
            args.push(reason);
        }

        self.run(&args).await.and_then(|response| {
            if response == INVALID_BAN_FORMAT_MESSAGE.as_bytes() {
                Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    INVALID_BAN_FORMAT_MESSAGE,
                ))
            } else {
                Ok(())
            }
        })
    }

    async fn remove_ban(&mut self, id: u64) -> std::io::Result<()> {
        self.run(&["removeBan".into(), id.to_string().into()])
            .await
            .map(drop)
    }
}

impl<T> Broadcast for T
where
    T: DayZ + Send,
{
    async fn broadcast(&mut self, message: Cow<'_, str>) -> std::io::Result<()> {
        self.say(BROADCAST_TARGET, message).await
    }
}

impl<T> Players for T
where
    T: DayZ + Send,
{
    type Player = Player;

    async fn players(&mut self) -> std::io::Result<Vec<Self::Player>> {
        let result = self.run(&["players".into()]).await?;
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
