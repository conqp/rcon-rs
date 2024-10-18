mod handler;

use crate::battleye::client::handler::Handler;
use crate::battleye::packet::{command, login, CommunicationResult, Request, Response};
use crate::RCon;
use std::borrow::Cow;
use std::io;
use std::io::ErrorKind;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{lookup_host, ToSocketAddrs};
use tokio::spawn;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use udp_stream::UdpStream;

const DEFAULT_HANDLER_INTERVAL: Duration = Duration::from_secs(1);
const DEFAULT_CHANNEL_BUFFER: usize = 8;

/// A `BattlEye Rcon` client.
#[derive(Debug)]
pub struct Client {
    running: Arc<AtomicBool>,
    requests: Sender<Request>,
    responses: Receiver<Response>,
    handler: Option<JoinHandle<()>>,
    buffer: Vec<command::Response>,
}

impl Client {
    /// Creates a new instance of the client.
    #[must_use]
    pub fn new<const CHANNEL_BUFFER: usize>(udp_stream: UdpStream) -> Self {
        Self::new_with_handler_interval::<CHANNEL_BUFFER>(
            udp_stream,
            Some(DEFAULT_HANDLER_INTERVAL),
        )
    }

    /// Creates a new instance of the client with a custom handler interval set.
    #[must_use]
    pub fn new_with_handler_interval<const CHANNEL_BUFFER: usize>(
        udp_stream: UdpStream,
        handler_interval: Option<Duration>,
    ) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let (request_tx, request_rx) = channel(CHANNEL_BUFFER);
        let (response_tx, response_rx) = channel(CHANNEL_BUFFER);
        let handler = Handler::new(
            udp_stream,
            running.clone(),
            request_rx,
            response_tx,
            handler_interval,
        );
        let join_handle = spawn(async { handler.run().await });
        Self {
            running,
            requests: request_tx,
            responses: response_rx,
            handler: Some(join_handle),
            buffer: Vec::new(),
        }
    }

    async fn communicate(&mut self, request: Request) -> io::Result<CommunicationResult> {
        self.requests
            .send(request)
            .await
            .map_err(|_| ErrorKind::BrokenPipe)?;

        self.buffer.clear();

        loop {
            if let Some(response) = self.responses.recv().await {
                match response {
                    Response::Command(response) => {
                        let seq = response.seq() as usize;
                        self.buffer.push(response);

                        if self.buffer.len() >= seq {
                            return Ok(CommunicationResult::Command(self.collect_responses()));
                        }
                    }
                    Response::Login(response) => return Ok(CommunicationResult::Login(response)),
                }
            }
        }
    }

    fn collect_responses(&mut self) -> Vec<u8> {
        self.buffer.sort_by_key(command::Response::seq);
        self.buffer
            .iter()
            .flat_map(command::Response::payload)
            .copied()
            .collect()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.running.store(false, SeqCst);

        if let Some(handler) = self.handler.take() {
            handler.abort();
        }
    }
}

impl RCon for Client {
    async fn connect<T>(address: T) -> io::Result<Self>
    where
        T: ToSocketAddrs + Send,
    {
        if let Some(address) = lookup_host(address).await?.next() {
            return UdpStream::connect(address)
                .await
                .map(Self::new::<DEFAULT_CHANNEL_BUFFER>);
        }

        Err(io::Error::other("No host found."))
    }

    async fn login(&mut self, password: &str) -> io::Result<bool> {
        match self
            .communicate(Request::Login(login::Request::from(password)))
            .await?
        {
            CommunicationResult::Login(response) => Ok(response.success()),
            CommunicationResult::Command(_) => Err(io::Error::new(
                ErrorKind::InvalidData,
                "Expected login response, but got a command response.",
            )),
        }
    }

    async fn run<'a>(&mut self, args: &[Cow<'a, str>]) -> io::Result<Vec<u8>> {
        let command = args.join(" ");

        match self
            .communicate(Request::Command(command::Request::from(command.as_str())))
            .await?
        {
            CommunicationResult::Command(bytes) => Ok(bytes),
            CommunicationResult::Login(_) => Err(io::Error::new(
                ErrorKind::InvalidData,
                "Expected command response, but got a login response.",
            )),
        }
    }
}
