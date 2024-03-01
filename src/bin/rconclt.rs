use clap::Parser;
use log::error;
use rcon::{source::Client, RCon};
use std::io::{stdout, Write};
use std::net::TcpStream;
use std::process::exit;
use std::time::Duration;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    password: String,
    #[arg(short, long, help = "timeout in milliseconds")]
    timeout: Option<u64>,
    #[arg(index = 1)]
    server: String,
    #[arg(index = 2)]
    command: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let tcp_stream = TcpStream::connect(&args.server).unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });

    tcp_stream
        .set_read_timeout(args.timeout.map(Duration::from_millis))
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(2);
        });

    let mut client: Client = tcp_stream.into();
    let logged_in = client.login(&args.password).unwrap_or_else(|error| {
        error!("{error}");
        exit(3);
    });
    if logged_in {
        let result = client.run(&args.command).unwrap_or_else(|error| {
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
