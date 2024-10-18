//! An example `RCON` client supporting both `Source RCON` and `BattlEye Rcon`.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::process::exit;
use std::time::Duration;

use clap::Parser;
use log::error;
use rcon::{battleye, RCon};
use rpassword::prompt_password;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
struct Args {
    #[arg(index = 1, help = "The server address to connect to")]
    server: SocketAddr,
    #[arg(short, long, help = "Connection timeout in seconds", default_value_t = DEFAULT_TIMEOUT.as_secs())]
    timeout: u64,
    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,
    #[arg(help = "The command to execute")]
    command: Vec<Cow<'static, str>>,
}

impl Args {
    fn client(&self) -> std::io::Result<battleye::Client> {
        UdpSocket::bind(if self.server.is_ipv4() {
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
        } else {
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0)
        })
        .and_then(|socket| {
            socket.set_read_timeout(Some(self.timeout()))?;
            socket.set_write_timeout(Some(self.timeout()))?;
            socket.connect(self.server)?;
            Ok(socket)
        })
        .map(battleye::Client::new)
    }

    const fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }

    fn password(&self) -> std::io::Result<String> {
        self.password.as_ref().map_or_else(
            || prompt_password("Enter password: "),
            |password| Ok(password.clone()),
        )
    }
}

// TODO: Implement DayZ specific features.
fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut client = args.client().unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });

    let logged_in = args
        .password()
        .and_then(|password| client.login(password.into()))
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(3);
        });

    if logged_in {
        let result = client.run(args.command.as_slice()).unwrap_or_else(|error| {
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
