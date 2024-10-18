use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use log::{debug, error, trace, warn};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;
use udp_stream::UdpStream;

use crate::battleye::from_server::FromServer;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;
use crate::battleye::packet::server::{Ack, Message};
use crate::battleye::packet::{command, login, server, Request, Response};

/// Idle timeout according to protocol definition: <https://www.battleye.com/downloads/BERConProtocol.txt>
const IDLE_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Debug)]
pub struct Handler {
    udp_stream: UdpStream,
    running: Arc<AtomicBool>,
    requests: Receiver<Request>,
    responses: Sender<Response>,
    interval: Option<Duration>,
    last_command: Option<SystemTime>,
}

impl Handler {
    #[must_use]
    pub const fn new(
        udp_stream: UdpStream,
        running: Arc<AtomicBool>,
        requests: Receiver<Request>,
        responses: Sender<Response>,
        interval: Option<Duration>,
    ) -> Self {
        Self {
            udp_stream,
            running,
            requests,
            responses,
            interval,
            last_command: None,
        }
    }

    pub async fn run(mut self) {
        while self.running.load(Relaxed) {
            match self.requests.try_recv() {
                Ok(request) => self.handle_request(request).await,
                Err(error) => match error {
                    TryRecvError::Disconnected => {
                        error!("Request channel disconnected");
                        return;
                    }
                    TryRecvError::Empty => {
                        self.process_incoming_messages().await;
                        self.keepalive().await;

                        if let Some(interval) = self.interval {
                            sleep(interval).await;
                        }
                    }
                },
            }
        }
    }

    async fn send(&mut self, request: Request) -> std::io::Result<()> {
        match request {
            Request::Command(request) => {
                self.udp_stream
                    .write_all(request.into_bytes().as_ref())
                    .await
            }
            Request::Login(request) => {
                self.udp_stream
                    .write_all(request.into_bytes().as_ref())
                    .await
            }
        }
    }

    async fn handle_request(&mut self, request: Request) {
        if let Err(error) = self.send(request).await {
            error!("{error}");
        }
    }

    async fn process_incoming_messages(&mut self) {
        if let Err(error) = self.receive().await {
            error!("{error}");
        }
    }

    async fn receive(&mut self) -> std::io::Result<()> {
        debug!("Receiving packet from UDP stream");
        let stream = &mut self.udp_stream;
        let header = Header::read_from(stream).await?;

        match header.typ() {
            command::TYPE => {
                let response = command::Response::read_from(stream)
                    .await
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Command)?;
                self.forward(response).await;
            }
            login::TYPE => {
                let response = login::Response::read_from(stream)
                    .await
                    .map(|f| f(header))
                    .and_then(FromServer::validate)
                    .map(Response::Login)?;
                self.forward(response).await;
            }
            server::TYPE => {
                let message = Message::read_from(stream)
                    .await
                    .map(|f| f(header))
                    .and_then(FromServer::validate)?;
                self.ack(message).await;
            }
            other => {
                error!("Received packet of invalid type: {other:#04X}");
            }
        };

        Ok(())
    }

    async fn forward(&self, response: Response) {
        debug!("Forwarding response from UDP stream");
        trace!("Response: {response:?}");

        if let Err(error) = self.responses.send(response).await {
            error!("Error sending response: {error}");
        }
    }

    async fn ack(&mut self, message: Message) {
        if let Err(error) = self
            .udp_stream
            .write_all(Ack::new(message.seq()).into_bytes().as_ref())
            .await
        {
            error!("Error sending ack: {error}");
        }
    }

    async fn keepalive(&mut self) {
        if self.needs_keepalive() {
            if let Err(error) = self.send(Self::keepalive_packet()).await {
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
