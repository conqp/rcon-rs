use std::borrow::Cow;
use std::io;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use log::{debug, trace};

use crate::battleye::client::handler::Handler;
use crate::battleye::packet::{command, login, CommunicationResult, Request, Response};
use crate::RCon;

mod handler;

const DEFAULT_HANDLER_INTERVAL: Duration = Duration::from_millis(100);
const DEFAULT_BUF_SIZE: usize = 1024;

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
    pub fn new(udp_socket: UdpSocket) -> Self {
        Self::new_ext(udp_socket, DEFAULT_BUF_SIZE, Some(DEFAULT_HANDLER_INTERVAL))
    }

    /// Creates a new instance of the client with additional information.
    #[must_use]
    pub fn new_ext(
        udp_socket: UdpSocket,
        buf_size: usize,
        handler_interval: Option<Duration>,
    ) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let (request_tx, request_rx) = channel();
        let (response_tx, response_rx) = channel();
        let handler = Handler::new(
            udp_socket,
            running.clone(),
            request_rx,
            response_tx,
            handler_interval,
            buf_size,
        );
        let join_handle = spawn(|| handler.run());
        Self {
            running,
            requests: request_tx,
            responses: response_rx,
            handler: Some(join_handle),
            buffer: Vec::new(),
        }
    }

    fn communicate(&mut self, request: Request) -> io::Result<CommunicationResult> {
        trace!("Sending request {:?}", request);
        self.requests
            .send(request)
            .map_err(|_| ErrorKind::BrokenPipe)?;

        debug!("Clearing buffer");
        self.buffer.clear();

        loop {
            debug!("Receiving response");
            match self.responses.recv() {
                Ok(response) => match response {
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
                Err(_) => return Err(ErrorKind::BrokenPipe.into()),
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
            handler.join().expect("Failed to join handler");
        }
    }
}

impl RCon for Client {
    fn login(&mut self, password: Cow<'_, str>) -> io::Result<bool> {
        match self.communicate(Request::Login(login::Request::from(password)))? {
            CommunicationResult::Login(response) => Ok(response.success()),
            CommunicationResult::Command(_) => Err(io::Error::new(
                ErrorKind::InvalidData,
                "Expected login response, but got a command response.",
            )),
        }
    }

    fn run(&mut self, args: &[Cow<'_, str>]) -> io::Result<Vec<u8>> {
        let command = args.join(" ");

        match self.communicate(Request::Command(command::Request::from(command.as_str())))? {
            CommunicationResult::Command(bytes) => Ok(bytes),
            CommunicationResult::Login(_) => Err(io::Error::new(
                ErrorKind::InvalidData,
                "Expected command response, but got a login response.",
            )),
        }
    }
}
