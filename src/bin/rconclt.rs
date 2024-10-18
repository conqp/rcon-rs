//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use clap::Parser;
use log::error;
use rcon::source::Quirks;
use rcon::{source::Client, RCon};
use std::borrow::Cow;
use std::io::{stdout, Write};
use std::process::exit;
use tokio::net::TcpStream;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    password: String,
    #[arg(short = 'P', long, help = "use quirks for Palword servers")]
    palworld: bool,
    #[arg(index = 1)]
    server: String,
    #[arg(index = 2)]
    command: Vec<Cow<'static, str>>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let tcp_stream = TcpStream::connect(&args.server)
        .await
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(1);
        });

    let mut client: Client = tcp_stream.into();

    if args.palworld {
        client = client.with_quirk(Quirks::PALWORLD);
    }

    let logged_in = client.login(&args.password).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(3);
    });
    if logged_in {
        let result = client.run(&args.command).await.unwrap_or_else(|error| {
            error!("{error}");
            exit(5);
        });
        stdout()
            .lock()
            .write_all(&result)
            .expect("Could not write result to stdout.");
    } else {
        error!("Login failed.");
        exit(4);
    }
}
