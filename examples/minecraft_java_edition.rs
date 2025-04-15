//! An example `RCON` client to test the extensions for `Minecraft: Java Edition`.

use std::io::{stdout, Write};
use std::net::SocketAddr;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::error;
use rcon::{minecraft::JavaEdition, source::Client, RCon};
use rpassword::prompt_password;

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
    #[command(about = "Get an attribute", name = "get-attribute")]
    GetAttribute {
        #[arg(help = "The target entity to read the attribute from")]
        target: String,
        #[arg(help = "The attribute to read")]
        attribute: String,
        #[arg(help = "An optional scaling factor")]
        scale: Option<f64>,
    },
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

    match client.login(&password).await {
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
        Command::GetAttribute {
            target,
            attribute,
            scale,
        } => client
            .attribute(target.into(), attribute.into())
            .get(scale)
            .await
            .and_then(|result| {
                stdout()
                    .lock()
                    .write_all(result.as_bytes())
                    .map_err(Into::into)
            }),
    } {
        error!("{error}");
        return ExitCode::from(5);
    }

    ExitCode::SUCCESS
}
