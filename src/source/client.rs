use std::net::SocketAddr;

use log::{debug, error, trace};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use super::packet::Packet;
use super::quirks::Quirks;
use super::server_data::ServerData;
use super::util::invalid_data;
use crate::RCon;

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

    async fn send(&mut self, packet: Packet) -> std::io::Result<()> {
        let bytes: Vec<_> = packet.try_into().map_err(invalid_data)?;
        debug!("Sending bytes: {bytes:?}");
        self.tcp_stream.write_all(bytes.as_slice()).await
    }

    async fn read_responses(
        &mut self,
        command_id: i32,
        sentinel_id: i32,
    ) -> std::io::Result<Vec<u8>> {
        loop {
            let packet = Packet::read_from(&mut self.tcp_stream).await?;

            match packet.typ {
                ServerData::ExecCommandOrAuthResponse => return Ok(packet.payload),
                ServerData::ResponseValue => {
                    if packet.typ == ServerData::ResponseValue {
                        if packet.id == sentinel_id {
                            debug!("Received sentinel packet");
                            return Ok(self.collect_buffer());
                        }

                        packet.validate(command_id, self.quirks)?;
                        debug!("Received data packet");
                        self.buffer.push(packet);
                    }
                }
                ServerData::Auth => {
                    error!("Received unexpected packet type: {:?}", ServerData::Auth);
                    trace!("Packet: {packet:?}");
                }
            }
        }
    }

    fn collect_buffer(&self) -> Vec<u8> {
        self.buffer
            .iter()
            .flat_map(|response| &response.payload)
            .copied()
            .collect()
    }
}

impl From<TcpStream> for Client {
    fn from(tcp_stream: TcpStream) -> Self {
        Self::new(tcp_stream)
    }
}

impl RCon for Client {
    async fn connect<T>(address: T) -> std::io::Result<Self>
    where
        Self: Sized,
        T: Into<SocketAddr> + Send,
    {
        TcpStream::connect(address.into()).await.map(Self::from)
    }

    async fn login<T>(&mut self, password: T) -> std::io::Result<bool>
    where
        T: AsRef<[u8]> + Send,
    {
        self.send(Packet::login(password.as_ref())).await?;
        let mut packet;

        loop {
            debug!("Reading response packet.");
            packet = Packet::read_from(&mut self.tcp_stream).await?;
            if packet.typ == ServerData::ExecCommandOrAuthResponse {
                break;
            }
        }

        Ok(packet.id >= 0)
    }

    async fn run<T>(&mut self, args: T) -> std::io::Result<Vec<u8>>
    where
        T: AsRef<[u8]> + Send,
    {
        let command = Packet::command(args.as_ref());
        let command_id = command.id;
        let sentinel = Packet::sentinel(command.id);
        let sentinel_id = sentinel.id;
        self.send(command).await?;
        self.send(sentinel).await?;
        self.read_responses(command_id, sentinel_id).await
    }
}
