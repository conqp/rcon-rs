//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::net::{IpAddr, SocketAddr};
use std::process::ExitCode;
use std::time::Duration;

use clap::{Parser, Subcommand};
use log::error;
use rcon::{battleye::Client, Ban, BanList, Broadcast, Kick, Player, Players, RCon, Say, Target};
use rpassword::prompt_password;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
struct Args {
    #[arg(index = 1, help = "The server address to connect to")]
    server: SocketAddr,
    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
#[command(subcommand_value_name = "COMMAND")]
enum Command {
    #[command(about = "List players on the server", name = "players")]
    Players,
    #[command(
        about = "Send a message directly to all players on the server",
        name = "say-to-all"
    )]
    SayToAll {
        #[arg(help = "The message")]
        message: Cow<'static, str>,
    },
    #[command(about = "Send a message to a player", name = "say")]
    Say {
        #[arg(help = "The player to send the message to")]
        player: Cow<'static, str>,
        #[arg(help = "The message")]
        message: Cow<'static, str>,
    },
    #[command(about = "Send a broadcast message to all players", name = "broadcast")]
    Broadcast {
        #[arg(help = "The message")]
        message: Cow<'static, str>,
    },
    #[command(about = "Kick a player from the server", name = "kick")]
    Kick {
        #[arg(help = "The player to kick")]
        player: Cow<'static, str>,
        #[arg(short, long, help = "An optional reason for the kick")]
        reason: Option<Cow<'static, str>>,
    },
    #[command(
        about = "Kick a player from the server given their name",
        name = "kick-by-name"
    )]
    KickByName {
        #[arg(help = "The name of the player to kick")]
        name: Cow<'static, str>,
        #[arg(short, long, help = "An optional reason for the kick")]
        reason: Option<Cow<'static, str>>,
    },
    #[command(
        about = "Kick a player from the server given their UUID",
        name = "kick-by-uuid"
    )]
    KickByUuid {
        #[arg(help = "The UUID of the player to kick")]
        uuid: Uuid,
        #[arg(short, long, help = "An optional reason for the kick")]
        reason: Option<Cow<'static, str>>,
    },
    #[command(about = "Ban a player from the server", name = "ban")]
    Ban {
        #[arg(help = "The player to ban")]
        player: Cow<'static, str>,
        #[arg(short, long, help = "An optional reason for the ban")]
        reason: Option<Cow<'static, str>>,
    },
    #[command(about = "Show the ban list", name = "bans")]
    Bans,
    #[command(about = "Add an entry to the ban list", name = "add-ban")]
    AddBan {
        #[clap(subcommand)]
        target: BanTarget,
        #[arg(help = "The duration of the ban in minutes")]
        duration: Option<u64>,
        #[arg(help = "The reason for the ban")]
        reason: Option<Cow<'static, str>>,
    },
    #[command(about = "Remove an entry from the ban list", name = "remove-ban")]
    RemoveBan {
        #[arg(help = "The Id of the entry to remove")]
        id: u64,
    },
    #[command(about = "Execute a raw command", name = "exec")]
    Exec {
        #[arg(help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
    },
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

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    let mut client = match Client::connect(args.server).await {
        Ok(client) => client,
        Err(error) => {
            error!("{error}");
            return ExitCode::from(1);
        }
    };

    let password = match args.password.as_ref().map_or_else(
        || prompt_password("Enter password: "),
        |password| Ok(password.clone()),
    ) {
        Ok(password) => password,
        Err(error) => {
            error!("{error}");
            return ExitCode::from(2);
        }
    };

    match client.login(password.into()).await {
        Ok(logged_in) => {
            if !logged_in {
                error!("Login failed.");
                return ExitCode::from(4);
            }
        }
        Err(error) => {
            error!("{error}");
            return ExitCode::from(3);
        }
    };

    if let Err(error) = match args.command {
        Command::Players => client
            .players()
            .await
            .map(|players| players.iter().for_each(|player| println!("{player}"))),
        Command::SayToAll { message } => say_to_all(&mut client, message).await,
        Command::Say { player, message } => client.say(player, message).await,
        Command::Broadcast { message } => client.broadcast(message).await,
        Command::Kick { player, reason } => client.kick(player, reason).await,
        Command::KickByName { name, reason } => kick_by_name(&mut client, name, reason).await,
        Command::KickByUuid { uuid, reason } => kick_by_uuid(&mut client, uuid, reason).await,
        Command::Ban { player, reason } => client.ban(player, reason).await,
        Command::Bans => client
            .bans()
            .await
            .map(|bans| bans.for_each(|ban| println!("{ban}"))),
        Command::AddBan {
            target,
            duration,
            reason,
        } => {
            client
                .add_ban(target.into(), duration.map(Duration::from_secs), reason)
                .await
        }
        Command::RemoveBan { id } => client.remove_ban(id).await,
        Command::Exec { command } => client
            .run(command.as_ref())
            .await
            .and_then(|result| stdout().lock().write_all(&result)),
    } {
        error!("{error}");
        return ExitCode::from(5);
    }

    ExitCode::SUCCESS
}

async fn say_to_all(client: &mut Client, message: Cow<'static, str>) -> std::io::Result<()> {
    match client.players_mut().await {
        Ok(mut players) => {
            while let Some(mut player) = players.next() {
                player.say(message.clone()).await.unwrap_or_else(|error| {
                    error!(
                        "Could not notify player #{} ({}): {error}",
                        player.id(),
                        player.name()
                    );
                });
            }

            Ok(())
        }
        Err(error) => Err(error),
    }
}

async fn kick_by_name(
    client: &mut Client,
    name: Cow<'static, str>,
    reason: Option<Cow<'static, str>>,
) -> std::io::Result<()> {
    let mut players = client.players_mut().await?;

    while let Some(mut player) = players.next() {
        if player.name() == name {
            player.kick(reason.clone()).await?;
        }
    }

    Ok(())
}

async fn kick_by_uuid(
    client: &mut Client,
    uuid: Uuid,
    reason: Option<Cow<'static, str>>,
) -> std::io::Result<()> {
    match client.players_mut().await {
        Ok(mut players) => {
            while let Some(mut player) = players.next() {
                if player.uuid() == Some(uuid) {
                    player
                        .kick(reason.clone())
                        .await
                        .unwrap_or_else(|error| error!(r#"Failed to kick player {uuid}: {error}"#));
                }
            }

            Ok(())
        }
        Err(error) => Err(error),
    }
}
