//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::process::ExitCode;

use clap::Parser;
use log::error;
use rcon::RCon;
use rcon::battleye::Client;
use rpassword::prompt_password;

use self::args::Args;

mod args;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    let mut client = match Client::connect(args.server()).await {
        Ok(client) => client,
        Err(error) => {
            error!("{error}");
            return ExitCode::from(1);
        }
    };

    let password = if let Some(password) = args.password() {
        Cow::Borrowed(password)
    } else {
        match prompt_password("Enter password: ") {
            Ok(password) => Cow::Owned(password),
            Err(error) => {
                error!("{error}");
                return ExitCode::from(2);
            }
        }
    };

    match client.login(password.as_bytes()).await {
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
    }

    if let Err(error) = args.run(&mut client).await {
        error!("{error}");
        return ExitCode::from(5);
    }

    ExitCode::SUCCESS
}
