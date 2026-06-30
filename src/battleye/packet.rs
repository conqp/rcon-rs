pub mod command;
pub mod login;
pub mod server;

/// Request packet types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request {
    /// A command request.
    Command(command::Request),

    /// A login request.
    Login(login::Request),
}

/// Response packet types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// A Command response.
    Command(command::Response),

    /// A login response.
    Login(login::Response),
}

/// A result of a client-server communication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommunicationResult {
    /// Result of a command.
    Command(Vec<u8>),

    /// Result of a login.
    Login(login::Response),
}
