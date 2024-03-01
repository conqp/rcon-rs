use async_std::net::TcpStream;
use clap::Parser;
use log::error;
use rcon::source::Fix;
use rcon::{source::Client, RCon};
use std::io::{stdout, Write};
use std::process::exit;
use std::time::Duration;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    password: String,
    #[arg(
        short = 't',
        long,
        help = "timeout in milliseconds for multi-packet responses"
    )]
    multi_packet_timeout: Option<u64>,
    #[arg(short = 'P', long, help = "use fixes for Palword servers")]
    palworld: bool,
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

    if args.palworld {
        client.fixes_mut().insert(Fix::Palworld);
    }

    let logged_in = client.login(&args.password).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(3);
    });
    if logged_in {
        let result = client
            .run(
                &args.command,
                args.multi_packet_timeout.map(Duration::from_millis),
            )
            .await
            .unwrap_or_else(|error| {
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
