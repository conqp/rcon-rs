use super::packet::Packet;
use super::server_data::ServerData;
use super::util::invalid_data;
use crate::source::fixes::Fix;
use crate::source::Fixes;
use crate::RCon;
use async_std::io::{timeout, WriteExt};
use async_std::net::{TcpStream, ToSocketAddrs};
use log::debug;
use std::collections::HashSet;
use std::io;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub struct Client {
    tcp_stream: TcpStream,
    fixes: Fixes,
    followup_timeout: Option<Duration>,
}

impl Client {
    #[must_use]
    pub const fn new(
        tcp_stream: TcpStream,
        fixes: Fixes,
        followup_timeout: Option<Duration>,
    ) -> Self {
        Self {
            tcp_stream,
            fixes,
            followup_timeout,
        }
    }

    #[must_use]
    pub const fn fixes(&self) -> &HashSet<Fix> {
        &self.fixes.0
    }

    pub fn fixes_mut(&mut self) -> &mut HashSet<Fix> {
        &mut self.fixes.0
    }

    #[must_use]
    pub const fn followup_timeout(&self) -> Option<Duration> {
        self.followup_timeout
    }

    pub fn set_followup_timeout(&mut self, followup_timeout: Option<Duration>) {
        self.followup_timeout = followup_timeout;
    }

    async fn send(&mut self, packet: Packet) -> io::Result<()> {
        let bytes: Vec<_> = packet.try_into().map_err(invalid_data)?;
        debug!("Sending bytes: {bytes:?}");
        self.tcp_stream.write_all(bytes.as_slice()).await
    }

    async fn read_responses(&mut self, id: i32) -> io::Result<Vec<Packet>> {
        let response = self.read_packet(id).await?;
        let mut responses = vec![response];

        if let Some(followup_timeout) = self.followup_timeout {
            while let Ok(response) =
                timeout(followup_timeout, async { self.read_packet(id).await }).await
            {
                responses.push(response);
            }
        }

        Ok(responses)
    }

    async fn read_packet(&mut self, id: i32) -> io::Result<Packet> {
        let packet = Packet::read_from(&mut self.tcp_stream).await?;

        if self.fixes.packet_is_valid(&packet, id) {
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
        Self::new(tcp_stream, Fixes::default(), None)
    }
}

impl RCon for Client {
    async fn connect<T>(address: T) -> io::Result<Self>
    where
        T: ToSocketAddrs + Send + Sync,
        <T as ToSocketAddrs>::Iter: Send + Sync,
    {
        TcpStream::connect(address)
            .await
            .map(|tcp_stream| Self::new(tcp_stream, Fixes::default(), None))
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

    async fn run<T>(&mut self, args: &[T]) -> io::Result<Arc<[u8]>>
    where
        T: AsRef<str> + Send + Sync,
    {
        let command = Packet::command(args);
        let id = command.id;
        self.send(command).await?;
        self.read_responses(id).await.map(|responses| {
            responses
                .into_iter()
                .flat_map(|response| response.payload)
                .collect()
        })
    }
}
