pub mod command;
pub mod login;
pub mod server;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request {
    Command(command::Request),
    Login(login::Request),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    Command(command::Response),
    Login(login::Response),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommunicationResult {
    Command(Vec<u8>),
    Login(login::Response),
}
