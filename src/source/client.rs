use log::{debug, error, trace};
use std::borrow::Cow;
use std::io;
use std::io::Write;
use std::net::TcpStream;

use super::packet::Packet;
use super::quirks::Quirks;
use super::server_data::ServerData;
use super::util::invalid_data;
use crate::RCon;

/// Multi-packet sentinel value: <https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Multiple-packet_Responses>
const SENTINEL: &[u8] = &[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];

/// A Source `RCON` client.
#[derive(Debug)]
pub struct Client {
    tcp_stream: TcpStream,
    quirks: Quirks,
    buffer: Vec<Packet>,
}

impl Client {
    /// Creates a new client instance.
    #[must_use]
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self {
            tcp_stream,
            quirks: Quirks::default(),
            buffer: Vec::new(),
        }
    }

    /// Returns the currently set quirks.
    #[must_use]
    pub const fn quirks(&self) -> Quirks {
        self.quirks
    }

    /// Enable a quirk on the client.
    pub fn enable_quirk(&mut self, quirk: Quirks) {
        self.quirks |= quirk;
    }

    /// Enable a quirk and return the client.
    #[must_use]
    pub fn with_quirk(mut self, quirk: Quirks) -> Self {
        self.enable_quirk(quirk);
        self
    }

    fn send(&mut self, packet: Packet) -> io::Result<()> {
        let bytes: Vec<_> = packet.try_into().map_err(invalid_data)?;
        debug!("Sending bytes: {bytes:?}");
        self.tcp_stream.write_all(bytes.as_slice())
    }

    fn read_responses(&mut self, command_id: i32, sentinel_id: i32) -> io::Result<Vec<u8>> {
        let mut sentinel_mirrored = false;

        loop {
            let packet = Packet::read_from(&mut self.tcp_stream)?;

            match packet.typ {
                ServerData::AuthResponse => return Ok(packet.payload),
                ServerData::ResponseValue => {
                    if packet.typ == ServerData::ResponseValue {
                        if packet.id == sentinel_id {
                            debug!("Received sentinel mirror packet");
                            sentinel_mirrored = true;
                            continue;
                        } else if sentinel_mirrored && packet.payload == SENTINEL {
                            debug!("Received sentinel payload packet");
                            return Ok(self
                                .buffer
                                .iter()
                                .flat_map(|response| &response.payload)
                                .copied()
                                .collect());
                        } else if packet.id == command_id || self.quirks.contains(Quirks::PALWORLD)
                        {
                            debug!("Received data packet");
                            self.buffer.push(packet);
                        } else {
                            return Err(invalid_data(format!(
                                "Packet ID mismatch: {} != {command_id}",
                                packet.id
                            )));
                        }
                    }
                }
                other => {
                    error!("Received unexpected packet type: {other:?}");
                    trace!("Packet: {packet:?}");
                }
            }
        }
    }
}

impl From<TcpStream> for Client {
    fn from(tcp_stream: TcpStream) -> Self {
        Self::new(tcp_stream)
    }
}

impl RCon for Client {
    fn login(&mut self, password: Cow<'_, str>) -> io::Result<bool> {
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

    fn run(&mut self, args: &[Cow<'_, str>]) -> io::Result<Vec<u8>> {
        let command = Packet::command(args);
        let command_id = command.id;
        let sentinel = Packet::sentinel(command.id);
        let sentinel_id = sentinel.id;
        self.send(command)?;
        self.send(sentinel)?;
        self.read_responses(command_id, sentinel_id)
    }
}
