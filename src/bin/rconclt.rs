use clap::Parser;
use log::error;
use rcon::source::Client;
use std::io::{stdout, Write};
use std::process::exit;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    password: String,
    #[arg(index = 1)]
    server: String,
    #[arg(index = 2)]
    command: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let mut client = Client::connect(&args.server).unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });
    let logged_in = client.login(&args.password).unwrap_or_else(|error| {
        error!("{error}");
        exit(2);
    });
    if logged_in {
        let result = client.run(&args.command).unwrap_or_else(|error| {
            error!("{error}");
            exit(4);
        });
        stdout()
            .lock()
            .write_all(&result)
            .expect("Could not write result to stdout.");
    } else {
        error!("Login failed.");
        exit(3);
    }
}
