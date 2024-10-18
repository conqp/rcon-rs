//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::process::exit;

use clap::Parser;
use log::error;

use rcon::{battleye, source, RCon};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    protocol: Protocol,
    #[arg(index = 1)]
    server: SocketAddr,
    #[arg(index = 2)]
    command: Vec<Cow<'static, str>>,
    #[arg(short, long)]
    password: Cow<'static, str>,
}

impl Args {
    pub fn client(&self) -> std::io::Result<Box<dyn RCon>> {
        match &self.protocol {
            Protocol::BattlEye => UdpSocket::bind(self.server)
                .map(battleye::Client::new)
                .map(|client| Box::new(client) as Box<dyn RCon>),
            Protocol::Source { quirks } => TcpStream::connect(self.server)
                .map(source::Client::new)
                .map(|client| {
                    if let Some(quirks) = quirks.iter().copied().reduce(|acc, quirk| acc | quirk) {
                        client.with_quirk(quirks)
                    } else {
                        client
                    }
                })
                .map(|client| Box::new(client) as Box<dyn RCon>),
        }
    }
}

#[derive(Debug, Parser)]
enum Protocol {
    Source {
        #[arg(short, long, help = "Enable quirks")]
        quirks: Vec<source::Quirks>,
    },
    BattlEye,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let mut client = args.client().unwrap_or_else(|error| {
        error!("{error}");
        exit(2);
    });

    let logged_in = client.login(args.password).unwrap_or_else(|error| {
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
