use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

use log::{debug, trace};
use tokio::io::AsyncWriteExt;
use tokio::spawn;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::task::JoinHandle;
use udp_stream::UdpStream;

use crate::battleye::client::handler::Handler;
use crate::battleye::into_bytes::IntoBytes;
use crate::battleye::packet::{command, login, CommunicationResult, Request, Response};
use crate::RCon;

mod handler;

const DEFAULT_CHANNEL_SIZE: usize = 8;

/// A `BattlEye Rcon` client.
#[derive(Debug)]
pub struct Client {
    udp_stream: UdpStream,
    running: Arc<AtomicBool>,
    responses: Receiver<std::io::Result<Response>>,
    handler: Option<JoinHandle<()>>,
    buffer: Vec<command::Response>,
}

impl Client {
    /// Creates a new instance of the client.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if connecting to the UDP server fails.
    pub async fn new<T>(address: T) -> std::io::Result<Self>
    where
        T: Into<SocketAddr> + Send,
    {
        Self::new_ext(address, DEFAULT_CHANNEL_SIZE).await
    }

    /// Creates a new instance of the client with additional information.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if connecting to the UDP server fails.
    pub async fn new_ext<T>(address: T, channel_size: usize) -> std::io::Result<Self>
    where
        T: Into<SocketAddr> + Send,
    {
        let address = address.into();
        let running = Arc::new(AtomicBool::new(true));
        let (response_tx, response_rx) = channel(channel_size);
        let handler = Handler::new(
            UdpStream::connect(address).await?,
            running.clone(),
            response_tx,
        );
        let join_handle = spawn(handler.run());
        UdpStream::connect(address).await.map(|udp_stream| Self {
            udp_stream,
            running,
            responses: response_rx,
            handler: Some(join_handle),
            buffer: Vec::new(),
        })
    }

    async fn communicate(&mut self, request: Request) -> std::io::Result<CommunicationResult> {
        trace!("Sending request {:?}", request);
        match request {
            Request::Command(command) => {
                self.udp_stream
                    .write_all(command.into_bytes().as_ref())
                    .await?;
            }
            Request::Login(login) => {
                self.udp_stream
                    .write_all(login.into_bytes().as_ref())
                    .await?;
            }
        }

        debug!("Clearing buffer");
        self.buffer.clear();

        loop {
            debug!("Receiving response");
            match self.responses.recv().await {
                Some(response) => match response? {
                    Response::Command(response) => {
                        debug!("Received command response");
                        trace!("Received response {:?}", response);
                        let seq = response.seq() as usize;
                        self.buffer.push(response);

                        if self.buffer.len() >= seq {
                            debug!("Buffer size exceeds sequence number. Returning.");
                            trace!("Buffer size: {}", self.buffer.len());
                            trace!("Sequence number: {}", seq);
                            return Ok(CommunicationResult::Command(self.collect_responses()));
                        }
                    }
                    Response::Login(response) => {
                        debug!("Received login response. Returning.");
                        trace!("Login response {:?}", response);
                        return Ok(CommunicationResult::Login(response));
                    }
                },
                None => return Err(ErrorKind::BrokenPipe.into()),
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
    async fn connect<T>(address: T) -> std::io::Result<Self>
    where
        Self: Sized,
        T: Into<SocketAddr> + Send,
    {
        Self::new(address).await
    }

    async fn login(&mut self, password: Cow<'_, str>) -> std::io::Result<bool> {
        match self
            .communicate(Request::Login(login::Request::from(password)))
            .await?
        {
            CommunicationResult::Login(response) => Ok(response.success()),
            CommunicationResult::Command(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Expected login response, but got a command response.",
            )),
        }
    }

    async fn run(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<Vec<u8>> {
        let command = args.join(" ");

        match self
            .communicate(Request::Command(command::Request::from(command.as_str())))
            .await?
        {
            CommunicationResult::Command(bytes) => Ok(bytes),
            CommunicationResult::Login(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Expected command response, but got a login response.",
            )),
        }
    }
}
