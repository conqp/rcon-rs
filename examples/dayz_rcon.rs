//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::net::SocketAddr;
use std::process::exit;

use clap::Parser;
use log::error;
use rcon::{battleye::Client, Ban, Bans, Broadcast, Kick, Player, Players, RCon, Say};
use rpassword::prompt_password;
use udp_stream::UdpStream;

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
    async fn client(&self) -> std::io::Result<Client> {
        UdpStream::connect(self.server).await.map(Client::new)
    }

    fn password(&self) -> std::io::Result<String> {
        self.password.as_ref().map_or_else(
            || prompt_password("Enter password: "),
            |password| Ok(password.clone()),
        )
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut client = args.client().await.unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });

    let password = args.password().unwrap_or_else(|error| {
        error!("{error}");
        exit(2);
    });

    let logged_in = client.login(password.into()).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(3);
    });

    if !logged_in {
        error!("Login failed.");
        exit(4);
    }

    match args.command {
        Command::Players => client
            .players()
            .await
            .map(|players| players.iter().for_each(|player| println!("{player:?}"))),
        Command::SayToAll { message } => match client.players_mut().await {
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
        },
        Command::Say { player, message } => client.say(player, message).await,
        Command::Broadcast { message } => client.broadcast(message).await,
        Command::Kick { player, reason } => client.kick(player, reason).await,
        Command::Ban { player, reason } => client.ban(player, reason).await,
        Command::Bans => client
            .bans()
            .await
            .map(|bans| bans.for_each(|ban| println!("{ban:?}"))),
        Command::Exec { command } => client
            .run(command.as_ref())
            .await
            .and_then(|result| stdout().lock().write_all(&result)),
    }
    .unwrap_or_else(|error| {
        error!("{error}");
    });
}
