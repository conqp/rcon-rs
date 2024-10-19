use std::error::Error;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use log::{debug, error, trace, warn};

use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;
use crate::battleye::packet::server::{Ack, Message};
use crate::battleye::packet::{command, login, server, Request, Response};

/// Idle timeout according to protocol definition: <https://www.battleye.com/downloads/BERConProtocol.txt>
const IDLE_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Debug)]
pub struct Handler {
    udp_socket: UdpSocket,
    running: Arc<AtomicBool>,
    requests: Receiver<Request>,
    responses: Sender<std::io::Result<Response>>,
    interval: Option<Duration>,
    last_command: Option<SystemTime>,
    buffer: Vec<u8>,
}

impl Handler {
    #[must_use]
    pub fn new(
        udp_socket: UdpSocket,
        running: Arc<AtomicBool>,
        requests: Receiver<Request>,
        responses: Sender<std::io::Result<Response>>,
        interval: Option<Duration>,
        buf_size: usize,
    ) -> Self {
        Self {
            udp_socket,
            running,
            requests,
            responses,
            interval,
            last_command: None,
            buffer: vec![0; buf_size],
        }
    }

    pub fn run(mut self) {
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
                        self.process_incoming_messages();
                        self.keepalive();

                        if let Some(interval) = self.interval {
                            debug!("Sleeping for {interval:?}");
                            sleep(interval);
                        }
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

    fn process_incoming_messages(&mut self) {
        debug!("Processing incoming messages");

        if let Err(error) = self.process_incoming_message_fallible() {
            debug!("Error while processing incoming messages: {error}");
            trace!("Error kind: {:?}", error.kind());
            trace!("Error source: {:?}", error.source());

            if error.kind() != ErrorKind::TimedOut && error.kind() != ErrorKind::WouldBlock {
                error!("Failed to receive message: {error}");
            }
        }
    }

    fn process_incoming_message_fallible(&mut self) -> std::io::Result<()> {
        let mut bytes = self.receive()?.iter().copied();
        debug!("Receiving packet from UDP stream");
        let header = Header::read_from(&mut bytes)?;
        trace!("Received header: {header:?}");

        match header.typ() {
            command::TYPE => {
                debug!("Received command response");
                let response = command::Response::read_from(&mut bytes)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Command);
                trace!("Command response: {response:?}");
                self.forward(response);
            }
            login::TYPE => {
                debug!("Received login response");
                let response = login::Response::read_from(&mut bytes)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Login);
                trace!("Login response: {response:?}");
                self.forward(response);
            }
            server::TYPE => {
                debug!("Received server message");
                let message = Message::read_from(&mut bytes)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)?;
                trace!("Server message: {message:?}");
                self.ack(&message);
            }
            other => {
                error!("Received packet of invalid type: {other:#04X}");
            }
        };

        Ok(())
    }

    fn receive(&mut self) -> std::io::Result<&[u8]> {
        let len = self.udp_socket.recv(&mut self.buffer)?;
        Ok(&self.buffer[..len])
    }

    fn forward(&self, response: std::io::Result<Response>) {
        debug!("Forwarding response from UDP stream");
        trace!("Response: {response:?}");

        if let Err(error) = self.responses.send(response) {
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

            if let Err(error) = self.send(Self::keepalive_packet()) {
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

    fn keepalive_packet() -> Request {
        Request::Command(command::Request::from(""))
    }
}
