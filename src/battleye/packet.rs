pub mod command;
pub mod login;
pub mod server;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Packet<'a> {
    Request(Request<'a>),
    Response(Response),
}

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
