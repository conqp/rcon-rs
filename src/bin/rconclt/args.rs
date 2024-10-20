use std::borrow::Cow;
use std::net::SocketAddr;

use clap::Parser;
use rcon::source;
use rpassword::prompt_password;

#[derive(Debug, Parser)]
#[command(author, version, about = "An RCon CLI client.")]
pub struct Args {
    #[arg(index = 1, help = "The server address to connect to")]
    server: SocketAddr,
    #[arg(short, long, help = "The password for the RCON server")]
    password: Option<String>,
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
