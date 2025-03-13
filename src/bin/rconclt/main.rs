//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::process::exit;

use args::Args;
use clap::Parser;
use log::error;
use rcon::{battleye, source, RCon};

use args::Protocol;

mod args;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let response = run(&args).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(3)
    });

    stdout().lock().write_all(&response)
}

async fn run(args: &Args) -> std::io::Result<Vec<u8>> {
    match args.protocol() {
        Protocol::BattlEye { command } => {
            let client = battleye::Client::connect(args.server()).await?;
            run_impl(client, args.password()?, command).await
        }
        Protocol::Source { command, quirks } => {
            let mut client = source::Client::connect(args.server()).await?;

            if let Some(quirks) = quirks.iter().copied().reduce(|acc, quirk| acc | quirk) {
                client.enable_quirk(quirks);
            }

            run_impl(client, args.password()?, command).await
        }
    }
}

async fn run_impl<T>(
    mut client: T,
    password: String,
    command: &[Cow<'static, str>],
) -> std::io::Result<Vec<u8>>
where
    T: RCon + Send,
{
    if !client.login(&password).await? {
        error!("Login failed.");
        exit(4);
    }

    client.run(command).await
}
