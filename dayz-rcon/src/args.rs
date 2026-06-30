use std::io::{Write, stdout};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use clap::{Parser, Subcommand};
use rcon::battleye::Client;
use rcon::dayz::{SECS_PER_MINUTE, Target};
use rcon::{DayZ, Error, RCon};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
pub struct Args {
    #[arg(index = 1, help = "The server address to connect to")]
    server: SocketAddr,

    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,

    #[clap(subcommand)]
    command: Command,
}

impl Args {
    /// Return the server socket.
    #[must_use]
    pub const fn server(&self) -> SocketAddr {
        self.server
    }

    /// Return the password.
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    /// Run the command.
    pub async fn run(self, client: &mut Client) -> Result<(), Error> {
        self.command.run(client).await
    }
}

#[derive(Debug, Subcommand)]
#[command(subcommand_value_name = "COMMAND")]
enum Command {
    #[command(about = "List players on the server", name = "players")]
    Players,
    #[command(about = "Send a message to a player", name = "say")]
    Say {
        #[arg(help = "The player to send the message to")]
        player: u64,
        #[arg(help = "The message")]
        message: String,
    },
    #[command(about = "Send a broadcast message to all players", name = "broadcast")]
    Broadcast {
        #[arg(help = "The message")]
        message: String,
    },
    #[command(about = "Kick a player from the server", name = "kick")]
    Kick {
        #[arg(help = "The player to kick")]
        player: u64,
        #[arg(short, long, help = "An optional reason for the kick")]
        reason: Option<String>,
    },
    #[command(about = "Ban a player from the server", name = "ban")]
    Ban {
        #[arg(help = "The player to ban")]
        player: u64,
        #[arg(short, long, help = "An optional reason for the ban")]
        reason: Option<String>,
    },
    #[command(about = "Show the ban list", name = "bans")]
    Bans,
    #[command(about = "Add an entry to the ban list", name = "add-ban")]
    AddBan {
        #[clap(subcommand)]
        target: BanTarget,
        #[arg(short, long, help = "The duration of the ban in minutes")]
        duration: Option<u64>,
        #[arg(short, long, help = "The reason for the ban")]
        reason: Option<String>,
    },
    #[command(about = "Remove an entry from the ban list", name = "remove-ban")]
    RemoveBan {
        #[arg(help = "The Id of the entry to remove")]
        id: u64,
    },
    #[command(about = "Execute a raw command", name = "exec")]
    Exec {
        #[arg(help = "The command to execute")]
        command: Vec<String>,
    },
}

impl Command {
    /// Run the command.
    async fn run(self, client: &mut Client) -> Result<(), Error> {
        match self {
            Self::Players => client
                .players()
                .await
                .map(|players| players.iter().for_each(|player| println!("{player}"))),
            Self::Say { player, message } => client.say(player, &message).await.map_err(Into::into),
            Self::Broadcast { message } => client.broadcast(&message).await.map_err(Into::into),
            Self::Kick { player, reason } => client.kick(player, reason).await.map_err(Into::into),
            Self::Ban { player, reason } => client.ban(player, reason).await.map_err(Into::into),
            Self::Bans => client
                .bans()
                .await
                .map(|bans| bans.iter().for_each(|ban| println!("{ban}"))),
            Self::AddBan {
                target,
                duration,
                reason,
            } => client
                .add_ban(
                    target.into(),
                    duration.map(|minutes| Duration::from_secs(minutes * SECS_PER_MINUTE)),
                    reason,
                )
                .await
                .map_err(Into::into),
            Self::RemoveBan { id } => client.remove_ban(id).await.map_err(Into::into),
            Self::Exec { command } => client
                .run(command.join(" "))
                .await
                .and_then(|result| stdout().lock().write_all(&result))
                .map_err(Into::into),
        }
    }
}

#[derive(Debug, Subcommand)]
#[command(subcommand_value_name = "TARGET")]
enum BanTarget {
    #[command(about = "Ban an IP address", name = "ip")]
    Ip {
        #[arg(help = "The IP address to ban")]
        ip: IpAddr,
    },
    #[command(about = "Ban a UUID", name = "uuid")]
    Uuid {
        #[arg(help = "The UUID to ban")]
        uuid: Uuid,
    },
}

impl From<BanTarget> for Target {
    fn from(target: BanTarget) -> Self {
        match target {
            BanTarget::Ip { ip } => Self::Ip(ip),
            BanTarget::Uuid { uuid } => Self::Uuid(uuid),
        }
    }
}
