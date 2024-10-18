use std::borrow::Cow;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use clap::Parser;
use rcon::{battleye, source, RCon, UdpSocketWrapper};
use rpassword::prompt_password;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

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
                let client = UdpSocketWrapper::connect(self.server).map(battleye::Client::new)?;
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
        #[arg(help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
        #[arg(short, long, help = "Enable quirks")]
        quirks: Vec<source::Quirks>,
    },
    #[command(about = "Use the BattlEys Rcon protocol", name = "battleye")]
    BattlEye {
        #[arg(help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
    },
}

impl Protocol {
    pub fn command(&self) -> &[Cow<'static, str>] {
        match self {
            Self::Source { command, .. } | Self::BattlEye { command, .. } => command.as_slice(),
        }
    }
}
