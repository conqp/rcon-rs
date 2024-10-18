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
    responses: Sender<Response>,
    interval: Option<Duration>,
    last_command: Option<SystemTime>,
}

impl Handler {
    #[must_use]
    pub const fn new(
        udp_socket: UdpSocket,
        running: Arc<AtomicBool>,
        requests: Receiver<Request>,
        responses: Sender<Response>,
        interval: Option<Duration>,
    ) -> Self {
        Self {
            udp_socket,
            running,
            requests,
            responses,
            interval,
            last_command: None,
        }
    }

    pub fn run(self) {
        while self.running.load(Relaxed) {
            match self.requests.try_recv() {
                Ok(request) => self.handle_request(request),
                Err(error) => match error {
                    TryRecvError::Disconnected => {
                        error!("Request channel disconnected");
                        return;
                    }
                    TryRecvError::Empty => {
                        self.process_incoming_messages();
                        self.keepalive();

                        if let Some(interval) = self.interval {
                            sleep(interval);
                        }
                    }
                },
            }
        }
    }

    fn send(&self, request: Request) -> std::io::Result<usize> {
        match request {
            Request::Command(request) => self.udp_socket.send(request.into_bytes().as_ref()),
            Request::Login(request) => self.udp_socket.send(request.into_bytes().as_ref()),
        }
    }

    fn handle_request(&self, request: Request) {
        if let Err(error) = self.send(request) {
            error!("{error}");
        }
    }

    fn process_incoming_messages(&self) {
        if let Err(error) = self.receive() {
            error!("{error}");
        }
    }

    fn receive(&self) -> std::io::Result<()> {
        debug!("Receiving packet from UDP stream");
        let header = Header::read_from(&self.udp_socket)?;

        match header.typ() {
            command::TYPE => {
                let response = command::Response::read_from(&self.udp_socket)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Command)?;
                self.forward(response);
            }
            login::TYPE => {
                let response = login::Response::read_from(&self.udp_socket)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Login)?;
                self.forward(response);
            }
            server::TYPE => {
                let message = Message::read_from(&self.udp_socket)
                    .map(|f| f(header))
                    .and_then(FromServer::validate)?;
                self.ack(&message);
            }
            other => {
                error!("Received packet of invalid type: {other:#04X}");
            }
        };

        Ok(())
    }

    fn forward(&self, response: Response) {
        debug!("Forwarding response from UDP stream");
        trace!("Response: {response:?}");

        if let Err(error) = self.responses.send(response) {
            error!("Error sending response: {error}");
        }
    }

    fn ack(&self, message: &Message) {
        if let Err(error) = self
            .udp_socket
            .send(Ack::new(message.seq()).into_bytes().as_ref())
        {
            error!("Error sending ack: {error}");
        }
    }

    fn keepalive(&self) {
        if self.needs_keepalive() {
            if let Err(error) = self.send(Self::keepalive_packet()) {
                error!("Error sending keepalive packet: {error}");
            }
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
