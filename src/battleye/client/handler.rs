use std::error::Error;
use std::io::ErrorKind;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use log::{debug, error, trace, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::Sender;
use udp_stream::UdpStream;

use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;
use crate::battleye::packet::server::{Ack, Message};
use crate::battleye::packet::{command, login, server, Response};

/// Idle timeout according to protocol definition: <https://www.battleye.com/downloads/BERConProtocol.txt>
const IDLE_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Debug)]
pub struct Handler {
    udp_stream: UdpStream,
    running: Arc<AtomicBool>,
    responses: Sender<std::io::Result<Response>>,
    last_command: Option<SystemTime>,
    buffer: Vec<u8>,
}

impl Handler {
    #[must_use]
    pub fn new(
        udp_stream: UdpStream,
        running: Arc<AtomicBool>,
        responses: Sender<std::io::Result<Response>>,
    ) -> Self {
        Self {
            udp_stream,
            running,
            responses,
            last_command: None,
            buffer: vec![0; 1024],
        }
    }

    pub async fn run(mut self) {
        while self.running.load(Relaxed) {
            trace!("Processing incoming message");
            self.process_incoming_message().await;
            trace!("Performing keepalive");
            self.keepalive().await;
        }
    }

    async fn process_incoming_message(&mut self) {
        debug!("Processing incoming messages");

        match self.receive_message().await {
            Ok(result) => {
                if let Some(response) = result {
                    self.forward(Ok(response)).await;
                }
            }
            Err(error) => {
                debug!("Error while processing incoming messages: {error}");
                trace!("Error kind: {:?}", error.kind());
                trace!("Error source: {:?}", error.source());

                if error.kind() != ErrorKind::TimedOut && error.kind() != ErrorKind::WouldBlock {
                    self.forward(Err(error)).await;
                }
            }
        }
    }

    async fn receive_message(&mut self) -> std::io::Result<Option<Response>> {
        trace!("Clearing buffer");
        self.buffer.clear();

        debug!("Receiving packet from UDP socket");
        let bytes = self.udp_stream.read(&mut self.buffer).await?;
        trace!("Received {bytes} bytes");

        if bytes == 0 {
            debug!("Received 0 bytes");
            return Ok(None);
        }

        trace!("Setting up byte stream");
        let mut stream = self.buffer.iter().take(bytes).copied();

        debug!("Parsing header from buffer");
        let header = Header::read_from(&mut stream)?;
        trace!("Received header: {header:?}");

        match header.typ() {
            command::TYPE => {
                debug!("Received command response");
                return command::Response::read_from(&mut stream)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Command)
                    .map(Some);
            }
            login::TYPE => {
                debug!("Received login response");
                return login::Response::read_from(&mut stream)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Login)
                    .map(Some);
            }
            server::TYPE => {
                debug!("Received server message");
                self.ack(
                    &Message::read_from(&mut stream)
                        .map(|f| f(header))
                        .and_then(FromServer::validate)?,
                )
                .await;
            }
            other => {
                error!("Received packet of invalid type: {other:#04X}");
            }
        };

        Ok(None)
    }

    async fn forward(&self, response: std::io::Result<Response>) {
        debug!("Forwarding response from UDP stream");
        trace!("Response: {response:?}");

        if let Err(error) = self.responses.send(response).await {
            error!("Error sending response: {error}");
        }
    }

    async fn ack(&mut self, message: &Message) {
        debug!("Ack'ing message #{}", message.seq());
        trace!("Message: {message:?}");

        if let Err(error) = self
            .udp_stream
            .write_all(Ack::new(message.seq()).into_bytes().as_ref())
            .await
        {
            error!("Error sending ack: {error}");
        }
    }

    async fn keepalive(&mut self) {
        debug!("Performing keepalive check");

        if self.needs_keepalive() {
            debug!("Need to send a keepalive message");

            if let Err(error) = self
                .udp_stream
                .write_all(command::Request::from("").into_bytes().as_ref())
                .await
            {
                error!("Error sending keepalive packet: {error}");
            }
        } else {
            debug!("No need to send keepalive message");
        }
    }

    fn needs_keepalive(&self) -> bool {
        self.last_command
            .and_then(|last_command| {
                last_command
                    .elapsed()
                    .inspect_err(|error| warn!("{error}"))
                    .ok()
                    .map(|elapsed| elapsed > IDLE_TIMEOUT / 2)
            })
            .unwrap_or_default()
    }
}
