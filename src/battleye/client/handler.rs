use std::error::Error;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use log::{debug, error, trace, warn};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;
use crate::battleye::packet::server::{Ack, Message};
use crate::battleye::packet::{command, login, server, Request, Response};

/// Idle timeout according to protocol definition: <https://www.battleye.com/downloads/BERConProtocol.txt>
const IDLE_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Debug)]
pub struct Handler<const BUFFER_SIZE: usize> {
    udp_socket: UdpSocket,
    seq: Arc<AtomicU8>,
    running: Arc<AtomicBool>,
    requests: Receiver<Request>,
    responses: Sender<std::io::Result<Response>>,
    last_command: Option<SystemTime>,
    buffer: [u8; BUFFER_SIZE],
}

impl<const BUFFER_SIZE: usize> Handler<BUFFER_SIZE> {
    #[must_use]
    pub const fn new(
        udp_socket: UdpSocket,
        seq: Arc<AtomicU8>,
        running: Arc<AtomicBool>,
        requests: Receiver<Request>,
        responses: Sender<std::io::Result<Response>>,
    ) -> Self {
        Self {
            udp_socket,
            seq,
            running,
            requests,
            responses,
            last_command: None,
            buffer: [0; BUFFER_SIZE],
        }
    }

    pub async fn run(mut self) {
        while self.running.load(Relaxed) {
            trace!("Receiving request");
            match self.requests.try_recv() {
                Ok(request) => {
                    trace!("Received request: {request:?}");
                    self.handle_request(request);
                }
                Err(error) => match error {
                    TryRecvError::Disconnected => {
                        error!("Request channel disconnected");
                        return;
                    }
                    TryRecvError::Empty => {
                        self.process_incoming_messages().await;
                        self.keepalive();
                    }
                },
            }
        }
    }

    fn handle_request(&self, request: Request) {
        trace!("Handling request: {request:?}");

        if let Err(error) = self.send(request) {
            error!("{error}");
        }
    }

    fn send(&self, request: Request) -> std::io::Result<usize> {
        trace!("Sending request: {request:?}");

        match request {
            Request::Command(request) => {
                let owner = request.into_bytes();
                let bytes = owner.as_ref();
                trace!("Sending bytes: {bytes:#04X?}");
                self.udp_socket.send(bytes)
            }
            Request::Login(request) => {
                let owner = request.into_bytes();
                let bytes = owner.as_ref();
                trace!("Sending bytes: {bytes:#04X?}");
                self.udp_socket.send(bytes)
            }
        }
    }

    async fn process_incoming_messages(&mut self) {
        debug!("Processing incoming messages");

        match self.receive_response() {
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

    fn receive_response(&mut self) -> std::io::Result<Option<Response>> {
        debug!("Receiving packet from UDP socket");
        let bytes = self.udp_socket.recv(&mut self.buffer)?;
        trace!("Received {bytes} bytes");

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
                );
            }
            other => {
                error!("Received packet of invalid type: {other:#04X}");
            }
        }

        Ok(None)
    }

    async fn forward(&self, response: std::io::Result<Response>) {
        debug!("Forwarding response from UDP stream");
        trace!("Response: {response:?}");

        if let Err(error) = self.responses.send(response).await {
            error!("Error sending response: {error}");
        }
    }

    fn ack(&self, message: &Message) {
        debug!("Ack'ing message #{}", message.seq());
        trace!("Message: {message:?}");

        if let Err(error) = self
            .udp_socket
            .send(Ack::new(message.seq()).into_bytes().as_ref())
        {
            error!("Error sending ack: {error}");
        }
    }

    fn keepalive(&self) {
        debug!("Performing keepalive check");

        if self.needs_keepalive() {
            debug!("Need to send a keepalive message");

            if let Err(error) = self.send(self.keepalive_packet()) {
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

    fn keepalive_packet(&self) -> Request {
        Request::Command(command::Request::keepalive(self.seq.fetch_add(1, SeqCst)))
    }
}
