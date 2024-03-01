use super::packet::Packet;
use crate::source::server_data::ServerData;
use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug)]
pub struct Client {
    tcp_stream: TcpStream,
}

impl Client {
    #[must_use]
    pub const fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }

    /// Connect to the given socket address.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    pub fn connect(address: &SocketAddr) -> io::Result<Self> {
        TcpStream::connect(address).map(Self::new)
    }

    /// Perform a login.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    pub fn login(&mut self, password: &str) -> io::Result<bool> {
        self.send(Packet::login(password))?;

        let mut packet;

        loop {
            packet = Packet::read_from(&mut self.tcp_stream)?;
            if packet.typ == ServerData::AuthResponse {
                break;
            }
        }

        Ok(packet.id >= 0)
    }

    /// Run a command.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    pub fn run<T>(&mut self, args: &[T]) -> io::Result<Vec<u8>>
    where
        T: AsRef<str>,
    {
        let command = Packet::command(args);
        self.send(command)?;
        self.read_responses().map(|responses| {
            responses
                .into_iter()
                .flat_map(|response| response.payload)
                .collect()
        })
    }

    fn send(&mut self, packet: Packet) -> io::Result<()> {
        self.tcp_stream
            .write_all(Vec::<u8>::from(packet).as_slice())
    }

    fn read_responses(&mut self) -> io::Result<Vec<Packet>> {
        let response = Packet::read_from(&mut self.tcp_stream)?;
        let mut responses = vec![response];
        let read_timeout = self.tcp_stream.read_timeout()?;
        self.tcp_stream
            .set_read_timeout(Some(Duration::new(0, 0)))?;

        while let Ok(response) = Packet::read_from(&mut self.tcp_stream) {
            responses.push(response);
        }

        self.tcp_stream.set_read_timeout(read_timeout)?;
        Ok(responses)
    }
}

impl From<TcpStream> for Client {
    fn from(tcp_stream: TcpStream) -> Self {
        Self::new(tcp_stream)
    }
}
