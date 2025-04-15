//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::io::{stdout, Error, Write};
use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use log::error;
use rcon::{battleye, source, RCon};
use tokio::time::sleep;

use args::{Args, Protocol};

mod args;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    if let Err(code) = run(&args).await.and_then(|responses| {
        for response in responses {
            stdout()
                .lock()
                .write_all(&response)
                .map_err(io_error_to_exit_code)?;
        }

        Ok(())
    }) {
        return code;
    };

    ExitCode::SUCCESS
}

async fn run(args: &Args) -> Result<[Vec<u8>; 2], ExitCode> {
    match args.protocol() {
        Protocol::BattlEye { command } => {
            let client = battleye::Client::connect(args.server())
                .await
                .map_err(io_error_to_exit_code)?;
            run_impl(
                client,
                args.password().map_err(io_error_to_exit_code)?,
                command,
                args.delay(),
            )
            .await
        }
        Protocol::Source { command, quirks } => {
            let mut client = source::Client::connect(args.server())
                .await
                .map_err(io_error_to_exit_code)?;

            if let Some(quirks) = quirks.iter().copied().reduce(|acc, quirk| acc | quirk) {
                client.enable_quirk(quirks);
            }

            run_impl(
                client,
                args.password().map_err(io_error_to_exit_code)?,
                command,
                args.delay(),
            )
            .await
        }
    }
}

async fn run_impl<T>(
    mut client: T,
    password: String,
    command: &[String],
    delay: Duration,
) -> Result<[Vec<u8>; 2], ExitCode>
where
    T: RCon + Send,
{
    if !client
        .login(&password)
        .await
        .map_err(io_error_to_exit_code)?
    {
        error!("Login failed.");
        return Err(ExitCode::from(4));
    }

    let result1 = client.run(command).await.map_err(io_error_to_exit_code)?;
    sleep(delay).await;
    let result2 = client.run(command).await.map_err(io_error_to_exit_code)?;
    Ok([result1, result2])
}

#[allow(clippy::needless_pass_by_value)]
fn io_error_to_exit_code(error: Error) -> ExitCode {
    error!("{error}");
    ExitCode::from(5)
}
