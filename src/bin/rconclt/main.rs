//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::io::{stdout, Write};
use std::process::exit;

use clap::Parser;
use log::error;

use args::Args;

mod args;

fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut client = args.client().unwrap_or_else(|error| {
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
        let result = client.run(args.command()).unwrap_or_else(|error| {
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
