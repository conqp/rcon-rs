use clap::Parser;
use rcon::{battleye, source, RCon};
use rpassword::prompt_password;
use std::borrow::Cow;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream, UdpSocket};
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
pub struct Args {
    #[arg(index = 1, help = "The server address to connect to")]
    server: SocketAddr,
    #[arg(short, long, help = "Connection timeout in seconds", default_value_t = DEFAULT_TIMEOUT.as_secs())]
    timeout: u64,
    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,
    #[clap(subcommand)]
    protocol: Protocol,
}

impl Args {
    pub fn client(&self) -> std::io::Result<Box<dyn RCon>> {
        match &self.protocol {
            Protocol::BattlEye { .. } => {
                let client = UdpSocket::bind(if self.server.is_ipv4() {
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
                } else {
                    SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0)
                })
                .and_then(|socket| {
                    socket.set_read_timeout(Some(self.timeout()))?;
                    socket.set_write_timeout(Some(self.timeout()))?;
                    socket.connect(self.server).map(|()| socket)
                })
                .map(battleye::Client::new)?;
                Ok(Box::new(client))
            }
            Protocol::Source { quirks, .. } => {
                let client = TcpStream::connect(self.server)
                    .and_then(|socket| {
                        socket.set_read_timeout(Some(self.timeout()))?;
                        socket.set_write_timeout(Some(self.timeout()))?;
                        Ok(socket)
                    })
                    .map(source::Client::new)
                    .map(|client| {
                        if let Some(quirks) =
                            quirks.iter().copied().reduce(|acc, quirk| acc | quirk)
                        {
                            client.with_quirk(quirks)
                        } else {
                            client
                        }
                    })?;
                Ok(Box::new(client))
            }
        }
    }

    const fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }

    pub fn password(&self) -> std::io::Result<String> {
        self.password.as_ref().map_or_else(
            || prompt_password("Enter password: "),
            |password| Ok(password.clone()),
        )
    }

    pub fn command(&self) -> &[Cow<'static, str>] {
        self.protocol.command()
    }
}

#[derive(Debug, Parser)]
#[command(subcommand_value_name = "PROTOCOL")]
enum Protocol {
    #[command(about = "Use the Source RCON protocol", name = "source")]
    Source {
        #[arg(index = 2, help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
        #[arg(short, long, help = "Enable quirks")]
        quirks: Vec<source::Quirks>,
    },
    #[command(about = "Use the BattlEys Rcon protocol", name = "battleye")]
    BattlEye {
        #[arg(index = 2, help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
        #[arg(short, long, help = "Connection timeout in seconds")]
        timeout: u64,
    },
}

impl Protocol {
    pub fn command(&self) -> &[Cow<'static, str>] {
        match self {
            Self::Source { command, .. } | Self::BattlEye { command, .. } => command.as_slice(),
        }
    }
}
