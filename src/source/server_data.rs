#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ServerData {
    Auth,
    AuthResponse,
    ExecCommand,
    ResponseValue,
}

impl From<ServerData> for i32 {
    fn from(typ: ServerData) -> Self {
        match typ {
            ServerData::Auth => 3,
            ServerData::ExecCommand | ServerData::AuthResponse => 2,
            ServerData::ResponseValue => 0,
        }
    }
}

impl TryFrom<i32> for ServerData {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::Auth),
            2 => Ok(Self::AuthResponse),
            0 => Ok(Self::ResponseValue),
            other => Err(other),
        }
    }
}
