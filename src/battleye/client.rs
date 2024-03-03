use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use crate::battleye::packet::server::Message;
use crate::battleye::packet::{command, login, server, CommunicationResult, Request, Response};
use crate::battleye::to_server::ToServer;
use crate::RCon;
use log::warn;
use std::io;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{lookup_host, ToSocketAddrs};
use tokio::time::timeout;
use udp_stream::UdpStream;

pub struct Client {
    udp_stream: UdpStream,
    handler: Option<Sender<Message>>,
}

impl Client {
    #[must_use]
    pub const fn new(udp_stream: UdpStream, handler: Option<Sender<Message>>) -> Self {
        Self {
            udp_stream,
            handler,
        }
    }

    /// Acknowledge a server message.
    ///
    /// # Errors
    /// Returns an [`io::Error`] if the ack message could not be sent.
    pub async fn ack_message(&mut self, seq: u8) -> io::Result<()> {
        self.send(Request::Server(server::Ack::new(seq))).await
    }

    async fn communicate<'request>(
        &mut self,
        request: Request<'request>,
        multi_packet_timeout: Option<Duration>,
    ) -> io::Result<CommunicationResult> {
        self.send(request).await?;
        let mut responses = Vec::new();

        while let Ok(packet) = if let Some(t) = multi_packet_timeout {
            timeout(t, async { self.receive().await }).await?
        } else {
            self.receive().await
        } {
            match packet {
                Response::Command(response) => responses.push(response),
                Response::Login(response) => return Ok(CommunicationResult::Login(response)),
                Response::Server(message) => {
                    if let Some(handler) = &mut self.handler {
                        handler.send(message).map_err(io::Error::other)?;
                    } else {
                        warn!("Received server response, but no handler was provided.");
                    }
                }
            }
        }

        if responses.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No data received.",
            ))
        } else {
            responses.sort_by_key(command::Response::seq);
            Ok(CommunicationResult::CommandResult(
                responses
                    .iter()
                    .flat_map(command::Response::payload)
                    .copied()
                    .collect(),
            ))
        }
    }

    async fn send<'request>(&mut self, request: Request<'request>) -> io::Result<()> {
        match request {
            Request::Command(request) => request.write_to(&mut self.udp_stream).await,
            Request::Login(request) => request.write_to(&mut self.udp_stream).await,
            Request::Server(ack) => ack.write_to(&mut self.udp_stream).await,
        }
    }

    async fn receive(&mut self) -> io::Result<Response> {
        let header = Header::read_from(&mut self.udp_stream).await?;

        match header.typ() {
            command::TYPE => command::Response::read_from(&mut self.udp_stream)
                .await
                .map(|f| f(header))
                .and_then(FromServer::validate)
                .map(Response::Command),
            login::TYPE => login::Response::read_from(&mut self.udp_stream)
                .await
                .map(|f| f(header))
                .and_then(FromServer::validate)
                .map(Response::Login),

            server::TYPE => Message::read_from(&mut self.udp_stream)
                .await
                .map(|f| f(header))
                .and_then(FromServer::validate)
                .map(Response::Server),
            other => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid packet type: {other}"),
            )),
        }
    }
}

impl RCon for Client {
    async fn connect<T>(address: T) -> io::Result<Self>
    where
        T: ToSocketAddrs + Send + Sync,
    {
        if let Some(address) = lookup_host(address).await?.next() {
            return UdpStream::connect(address)
                .await
                .map(|udp_stream| Self::new(udp_stream, None));
        }

        Err(io::Error::other("No host found."))
    }

    async fn login(&mut self, password: &str) -> io::Result<bool> {
        match self
            .communicate(Request::Login(login::Request::from(password)), None)
            .await?
        {
            CommunicationResult::Login(response) => Ok(response.success()),
            CommunicationResult::CommandResult(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected login response, but got a command result.",
            )),
        }
    }

    async fn run<T>(
        &mut self,
        args: &[T],
        multi_packet_timeout: Option<Duration>,
    ) -> io::Result<Arc<[u8]>>
    where
        T: AsRef<str> + Send + Sync,
    {
        let command = args.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(" ");

        match self
            .communicate(
                Request::Command(command::Request::from(command.as_str())),
                multi_packet_timeout,
            )
            .await?
        {
            CommunicationResult::CommandResult(bytes) => Ok(bytes),
            CommunicationResult::Login(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected login response, but got a login response.",
            )),
        }
    }
}
