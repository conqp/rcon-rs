//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::process::exit;
use std::time::Duration;

use clap::Parser;
use log::error;
use rcon::{battleye, Ban, Bans, Broadcast, Kick, Player, Players, RCon, Say};
use rpassword::prompt_password;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
struct Args {
    #[arg(index = 1, help = "The server address to connect to")]
    server: SocketAddr,
    #[arg(short, long, help = "Connection timeout in seconds", default_value_t = DEFAULT_TIMEOUT.as_secs())]
    timeout: u64,
    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
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
    #[command(about = "Ban a player from the server", name = "ban")]
    Ban {
        #[arg(help = "The player to ban")]
        player: Cow<'static, str>,
        #[arg(short, long, help = "An optional reason for the ban")]
        reason: Option<Cow<'static, str>>,
    },
    #[command(about = "Show the ban list", name = "bans")]
    Bans,
    #[command(about = "Execute a raw command", name = "exec")]
    Exec {
        #[arg(help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
    },
}

impl Args {
    fn client(&self) -> std::io::Result<battleye::Client> {
        UdpSocket::bind(if self.server.is_ipv4() {
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
        } else {
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0)
        })
        .and_then(|socket| {
            socket.set_read_timeout(Some(self.timeout()))?;
            socket.set_write_timeout(Some(self.timeout()))?;
            socket.connect(self.server)?;
            Ok(socket)
        })
        .map(battleye::Client::new)
        .map(Into::into)
    }

    const fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }

    fn password(&self) -> std::io::Result<String> {
        self.password.as_ref().map_or_else(
            || prompt_password("Enter password: "),
            |password| Ok(password.clone()),
        )
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut client = args.client().unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });

    let logged_in = args
        .password()
        .and_then(|password| client.login(password.into()))
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(3);
        });

    if logged_in {
        match args.command {
            Command::Players => client
                .players()
                .map(|players| players.iter().for_each(|player| println!("{player:?}"))),
            Command::SayToAll { message } => client.players_mut().map(|mut players| {
                while let Some(mut player) = players.next() {
                    player.say(message.clone()).unwrap_or_else(|error| {
                        error!(
                            "Could not notify player #{} ({}): {error}",
                            player.id(),
                            player.name()
                        );
                    });
                }
            }),
            Command::Say { player, message } => client.say(player, message),
            Command::Broadcast { message } => client.broadcast(message),
            Command::Kick { player, reason } => client.kick(player, reason),
            Command::Ban { player, reason } => client.ban(player, reason),
            Command::Bans => client
                .bans()
                .map(|bans| bans.for_each(|ban| println!("{ban:?}"))),
            Command::Exec { command } => client
                .run(command.as_ref())
                .and_then(|result| stdout().lock().write_all(&result)),
        }
        .unwrap_or_else(|error| {
            error!("{error}");
        });
    } else {
        error!("Login failed.");
        exit(4);
    }
}
