use crate::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerData {
    Auth,
    AuthResponse,
    ExecCommand,
    ResponseValue,
}

impl From<ServerData> for i32 {
    fn from(packet_type: ServerData) -> Self {
        match packet_type {
            ServerData::Auth => 3,
            ServerData::AuthResponse => 2,
            ServerData::ExecCommand => 2,
            ServerData::ResponseValue => 0,
        }
    }
}

impl TryFrom<i32> for ServerData {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ResponseValue),
            2 => Ok(Self::AuthResponse),
            3 => Ok(Self::Auth),
            value => Err(Error::InvalidServerData(value)),
        }
    }
}
