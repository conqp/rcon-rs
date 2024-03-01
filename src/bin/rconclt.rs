use async_std::net::TcpStream;
use clap::Parser;
use log::error;
use rcon::{source::Client, RCon};
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
