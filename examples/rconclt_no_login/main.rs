//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::io::{stdout, Error, Write};
use std::process::ExitCode;

use args::Args;
use clap::Parser;
use log::error;
use rcon::{battleye, source, RCon};

use args::Protocol;

mod args;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    if let Err(code) = run(&args).await.and_then(|response| {
        stdout()
            .lock()
            .write_all(&response)
            .map_err(io_error_to_exit_code)
    }) {
        return code;
    };

    ExitCode::SUCCESS
}

async fn run(args: &Args) -> Result<Vec<u8>, ExitCode> {
    match args.protocol() {
        Protocol::BattlEye { command } => {
            let mut client = battleye::Client::connect(args.server())
                .await
                .map_err(io_error_to_exit_code)?;
            client
                .run(command.join(" "))
                .await
                .map_err(io_error_to_exit_code)
        }
        Protocol::Source { command, quirks } => {
            let mut client = source::Client::connect(args.server())
                .await
                .map_err(io_error_to_exit_code)?;

            if let Some(quirks) = quirks.iter().copied().reduce(|acc, quirk| acc | quirk) {
                client.enable_quirk(quirks);
            }

            client
                .run(command.join(" "))
                .await
                .map_err(io_error_to_exit_code)
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn io_error_to_exit_code(error: Error) -> ExitCode {
    error!("{error}");
    ExitCode::from(5)
}
