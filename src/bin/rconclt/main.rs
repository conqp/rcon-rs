//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

mod args;

use clap::Parser;
use log::error;
use std::io::{stdout, Write};
use std::process::exit;

use args::Args;

fn main() {
    env_logger::init();
    let args = Args::parse();
    let protocol = args.protocol();
    let mut client = protocol.client().unwrap_or_else(|error| {
        error!("{error}");
        exit(2);
    });

    let logged_in = args
        .password()
        .and_then(|password| client.login(password.into()))
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(3);
        });

    if logged_in {
        let result = client.run(protocol.command()).unwrap_or_else(|error| {
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
