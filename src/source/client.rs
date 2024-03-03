use super::packet::Packet;
use super::quirks::{Quirk, Quirks};
use super::server_data::ServerData;
use super::util::invalid_data;
use crate::RCon;
use log::debug;
use std::collections::HashSet;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::time::timeout;

#[derive(Debug)]
pub struct Client {
    tcp_stream: TcpStream,
    quirks: Quirks,
}

impl Client {
    #[must_use]
    pub const fn new(tcp_stream: TcpStream, quirks: Quirks) -> Self {
        Self { tcp_stream, quirks }
    }

    #[must_use]
    pub const fn quirks(&self) -> &HashSet<Quirk> {
        &self.quirks.0
    }

    pub fn quirks_mut(&mut self) -> &mut HashSet<Quirk> {
        &mut self.quirks.0
    }

    #[must_use]
    pub fn with_quirk(mut self, quirk: Quirk) -> Self {
        self.quirks.0.insert(quirk);
        self
    }

    async fn send(&mut self, packet: Packet) -> io::Result<()> {
        let bytes: Vec<_> = packet.try_into().map_err(invalid_data)?;
        debug!("Sending bytes: {bytes:?}");
        self.tcp_stream.write_all(bytes.as_slice()).await
    }

    async fn read_responses(
        &mut self,
        id: i32,
        multi_packet_timeout: Option<Duration>,
    ) -> io::Result<Vec<Packet>> {
        let response = self.read_packet(id).await?;
        let mut responses = vec![response];

        if let Some(multi_packet_timeout) = multi_packet_timeout {
            while let Ok(response) =
                timeout(multi_packet_timeout, async { self.read_packet(id).await }).await?
            {
                responses.push(response);
            }
        }

        Ok(responses)
    }

    async fn read_packet(&mut self, id: i32) -> io::Result<Packet> {
        let packet = Packet::read_from(&mut self.tcp_stream).await?;

        if self.quirks.packet_is_valid(&packet, id) {
            Ok(packet)
        } else {
            Err(invalid_data(format!(
                "Packet ID mismatch: {} != {id}",
                packet.id
            )))
        }
    }
}

impl From<TcpStream> for Client {
    fn from(tcp_stream: TcpStream) -> Self {
        Self::new(tcp_stream, Quirks::default())
    }
}

impl RCon for Client {
    async fn connect<T>(address: T) -> io::Result<Self>
    where
        T: ToSocketAddrs + Send + Sync,
    {
        TcpStream::connect(address)
            .await
            .map(|tcp_stream| Self::new(tcp_stream, Quirks::default()))
    }

    async fn login(&mut self, password: &str) -> io::Result<bool> {
        self.send(Packet::login(password)).await?;
        let mut packet;

        loop {
            debug!("Reading response packet.");
            packet = Packet::read_from(&mut self.tcp_stream).await?;
            if packet.typ == ServerData::AuthResponse {
                break;
            }
        }

        Ok(packet.id >= 0)
    }

    async fn run<T>(
        &mut self,
        args: &[T],
        multi_packet_timeout: Option<Duration>,
    ) -> io::Result<Arc<[u8]>>
    where
        T: AsRef<str> + Send + Sync,
    {
        let command = Packet::command(args);
        let id = command.id;
        self.send(command).await?;
        self.read_responses(id, multi_packet_timeout)
            .await
            .map(|responses| {
                responses
                    .into_iter()
                    .flat_map(|response| response.payload)
                    .collect()
            })
    }
}
