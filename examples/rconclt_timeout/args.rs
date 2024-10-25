use std::borrow::Cow;
use std::net::SocketAddr;
use std::time::Duration;

use clap::Parser;
use rcon::source;
use rpassword::prompt_password;

const DEFAULT_DELAY: Duration = Duration::from_secs(60);

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
pub struct Args {
    #[arg(index = 1, help = "The server address to connect to")]
    server: SocketAddr,
    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,
    #[arg(short, long, help = "The delay in seconds", default_value_t = DEFAULT_DELAY.as_secs())]
    delay: u64,
    #[clap(subcommand)]
    protocol: Protocol,
}

impl Args {
    pub const fn server(&self) -> SocketAddr {
        self.server
    }

    pub fn password(&self) -> std::io::Result<String> {
        self.password.as_ref().map_or_else(
            || prompt_password("Enter password: "),
            |password| Ok(password.clone()),
        )
    }

    pub fn delay(&self) -> Duration {
        Duration::from_secs(self.delay)
    }

    pub const fn protocol(&self) -> &Protocol {
        &self.protocol
    }
}

#[derive(Debug, Parser)]
#[command(subcommand_value_name = "PROTOCOL")]
pub enum Protocol {
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
