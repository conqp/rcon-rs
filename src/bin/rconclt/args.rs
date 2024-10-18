use std::borrow::Cow;
use std::net::{SocketAddr, TcpStream, UdpSocket};

use clap::Parser;
use rcon::{battleye, source, RCon};
use rpassword::prompt_password;

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
pub struct Args {
    #[clap(subcommand)]
    protocol: Protocol,
    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,
}

impl Args {
    pub const fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn password(&self) -> std::io::Result<String> {
        self.password.as_ref().map_or_else(
            || prompt_password("Enter password: "),
            |password| Ok(password.clone()),
        )
    }
}

#[derive(Debug, Parser)]
#[command(subcommand_value_name = "PROTOCOL")]
pub enum Protocol {
    #[command(about = "Use the Source RCON protocol", name = "source")]
    Source {
        #[arg(index = 2, help = "The Source RCON server address")]
        server: SocketAddr,
        #[arg(index = 3, help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
        #[arg(short, long, help = "Enable quirks")]
        quirks: Vec<source::Quirks>,
    },
    #[command(about = "Use the BattlEys Rcon protocol", name = "battleye")]
    BattlEye {
        #[arg(index = 2, help = "The BattlEye Rcon server address")]
        server: SocketAddr,
        #[arg(index = 3, help = "The command to execute")]
        command: Vec<Cow<'static, str>>,
    },
}

impl Protocol {
    pub fn client(&self) -> std::io::Result<Box<dyn RCon>> {
        match self {
            Self::BattlEye { server, .. } => {
                let client = UdpSocket::bind(server).map(battleye::Client::new)?;
                Ok(Box::new(client))
            }
            Self::Source { server, quirks, .. } => {
                let client = TcpStream::connect(server)
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

    pub fn command(&self) -> &[Cow<'static, str>] {
        match self {
            Self::Source { command, .. } | Self::BattlEye { command, .. } => command.as_slice(),
        }
    }
}
