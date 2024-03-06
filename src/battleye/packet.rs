use std::sync::Arc;

pub mod command;
pub mod login;
pub mod server;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request<'a> {
    Command(command::Request<'a>),
    Login(login::Request<'a>),
    Server(server::Ack),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    Command(command::Response),
    Login(login::Response),
    Server(server::Message),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommunicationResult {
    Command(Arc<[u8]>),
    Login(login::Response),
}
