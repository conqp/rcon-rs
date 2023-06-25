use crate::packet::Packet;
use crate::{communicate, Error, ServerData};
use either::{Either, Right};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::io;
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    fragmentation_threshold: Option<usize>,
    rng: ThreadRng,
}

impl Client {
    #[must_use]
    pub fn new(stream: TcpStream, fragmentation_threshold: Option<usize>) -> Self {
        Self {
            stream,
            fragmentation_threshold,
            rng: rand::thread_rng(),
        }
    }

    /// Send a login request to the server
    /// # Errors
    /// Returns `Either<io::Error, rcon_rs::Error>` on either I/O or protocol errors
    pub fn login(&mut self, passwd: &str) -> Result<bool, Either<io::Error, Error>> {
        let login = Packet::new(self.rng.gen(), ServerData::Auth, passwd.as_bytes());
        let id = login.id();
        Ok(communicate(&mut self.stream, &login, self.fragmentation_threshold)?.id() == id)
    }

    /// Executes a command on the server and returns the reply as `Result<String, _>`
    /// # Errors
    /// Returns `Either<io::Error, rcon_rs::Error>` on either I/O or protocol errors
    pub fn exec(&mut self, command: &[&str]) -> Result<String, Either<io::Error, Error>> {
        let command = Packet::from(command);
        let response = communicate(&mut self.stream, &command, self.fragmentation_threshold)?;
        if response.id() == command.id() {
            Ok(response.text())
        } else {
            Err(Right(Error::NotLoggedIn))
        }
    }
}

impl From<TcpStream> for Client {
    fn from(stream: TcpStream) -> Self {
        Self::new(stream, None)
    }
}

impl FromStr for Client {
    type Err = io::Error;

    fn from_str(host: &str) -> Result<Self, Self::Err> {
        Ok(TcpStream::connect(host)?.into())
    }
}

impl TryFrom<(&str, u16)> for Client {
    type Error = io::Error;

    fn try_from((host, port): (&str, u16)) -> Result<Self, Self::Error> {
        Self::from_str(format!("{host}:{port}").as_str())
    }
}
