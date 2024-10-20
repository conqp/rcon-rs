//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::process::exit;

use crate::args::Protocol;
use args::Args;
use clap::Parser;
use log::error;
use rcon::{battleye, source, RCon};
use tokio::net::TcpStream;
use udp_stream::UdpStream;

mod args;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    make_client(&args).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(3)
    });
}

async fn make_client(args: &Args) -> std::io::Result<()> {
    match args.protocol() {
        Protocol::BattlEye { command } => {
            let client = UdpStream::connect(args.server())
                .await
                .map(battleye::Client::new)?;
            run(client, args.password()?, command).await
        }
        Protocol::Source { command, quirks } => {
            let client = TcpStream::connect(args.server())
                .await
                .map(source::Client::new)
                .map(|client| {
                    if let Some(quirks) = quirks.iter().copied().reduce(|acc, quirk| acc | quirk) {
                        client.with_quirk(quirks)
                    } else {
                        client
                    }
                })?;
            run(client, args.password()?, command).await
        }
    }
}

async fn run<T>(
    mut client: T,
    password: String,
    command: &[Cow<'static, str>],
) -> std::io::Result<()>
where
    T: RCon,
{
    if !client.login(password.into()).await? {
        error!("Login failed.");
        exit(4);
    }

    stdout().lock().write_all(&client.run(command).await?)
}
