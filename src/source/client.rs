use super::fixes::Fixes;
use super::packet::Packet;
use super::server_data::ServerData;
use super::util::invalid_data;
use crate::Rcon;
use log::debug;
use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

const FOLLOWUP_TIMEOUT: Duration = Duration::from_millis(1);

#[derive(Debug)]
pub struct Client {
    tcp_stream: TcpStream,
    fixes: HashSet<Fixes>,
    followup_timeout: Duration,
}

impl Client {
    #[must_use]
    pub const fn new(
        tcp_stream: TcpStream,
        fixes: HashSet<Fixes>,
        followup_timeout: Duration,
    ) -> Self {
        Self {
            tcp_stream,
            fixes,
            followup_timeout,
        }
    }

    /// Connect to the given socket address.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    pub fn connect<T>(address: T) -> io::Result<Self>
    where
        T: ToSocketAddrs,
    {
        TcpStream::connect(address)
            .map(|tcp_stream| Self::new(tcp_stream, HashSet::new(), FOLLOWUP_TIMEOUT))
    }

    #[must_use]
    pub const fn fixes(&self) -> &HashSet<Fixes> {
        &self.fixes
    }

    pub fn fixes_mut(&mut self) -> &mut HashSet<Fixes> {
        &mut self.fixes
    }

    #[must_use]
    pub const fn followup_timeout(&self) -> Duration {
        self.followup_timeout
    }

    pub fn set_followup_timeout(&mut self, followup_timeout: Duration) {
        self.followup_timeout = followup_timeout;
    }

    fn send(&mut self, packet: Packet) -> io::Result<()> {
        let bytes: Vec<_> = packet.try_into().map_err(invalid_data)?;
        debug!("Sending bytes: {bytes:?}");
        self.tcp_stream.write_all(bytes.as_slice())
    }

    fn read_responses(&mut self, id: i32) -> io::Result<Vec<Packet>> {
        let response = self.read_packet(id)?;
        let mut responses = vec![response];

        let read_timeout = self.tcp_stream.read_timeout()?;
        self.tcp_stream
            .set_read_timeout(Some(self.followup_timeout))?;

        while let Ok(response) = self.read_packet(id) {
            responses.push(response);
        }

        self.tcp_stream.set_read_timeout(read_timeout)?;
        Ok(responses)
    }

    fn read_packet(&mut self, id: i32) -> io::Result<Packet> {
        let packet = Packet::read_from(&mut self.tcp_stream)?;

        if self.packet_is_valid(&packet, id) {
            Ok(packet)
        } else {
            Err(invalid_data(format!(
                "Packet ID mismatch: {} != {id}",
                packet.id
            )))
        }
    }

    fn packet_is_valid(&self, packet: &Packet, id: i32) -> bool {
        if self.fixes.contains(&Fixes::Palworld) {
            return true;
        }

        packet.id == id
    }
}

impl From<TcpStream> for Client {
    fn from(tcp_stream: TcpStream) -> Self {
        Self::new(tcp_stream, HashSet::new(), FOLLOWUP_TIMEOUT)
    }
}

impl Rcon for Client {
    fn login(&mut self, password: &str) -> io::Result<bool> {
        self.send(Packet::login(password))?;
        let mut packet;

        loop {
            debug!("Reading response packet.");
            packet = Packet::read_from(&mut self.tcp_stream)?;
            if packet.typ == ServerData::AuthResponse {
                break;
            }
        }

        Ok(packet.id >= 0)
    }

    fn run<T>(&mut self, args: &[T]) -> io::Result<Vec<u8>>
    where
        T: AsRef<str>,
    {
        let command = Packet::command(args);
        let id = command.id;
        self.send(command)?;
        self.read_responses(id).map(|responses| {
            responses
                .into_iter()
                .flat_map(|response| response.payload)
                .collect()
        })
    }
}
