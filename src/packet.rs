use crate::server_data::ServerData;
use crate::Error;
use either::{Either, Left, Right};
use rand::random;
use std::io;
use std::io::Read;
use std::net::TcpStream;
use std::ops::AddAssign;

const TERMINATOR: [u8; 2] = [0, 0];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    id: i32,
    typ: ServerData,
    payload: Vec<u8>,
}

impl Packet {
    #[must_use]
    pub fn new(id: i32, typ: ServerData, payload: &[u8]) -> Self {
        Self {
            id,
            typ,
            payload: payload.into(),
        }
    }

    #[must_use]
    pub fn with_random_id(typ: ServerData, payload: &[u8]) -> Self {
        Self::new(random(), typ, payload)
    }

    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    #[must_use]
    pub const fn typ(&self) -> ServerData {
        self.typ
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        self.payload.as_slice()
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.payload.iter().map(|byte| *byte as char).collect()
    }
}

impl AddAssign for Packet {
    fn add_assign(&mut self, rhs: Self) {
        self.payload.extend_from_slice(&rhs.payload);
    }
}

impl From<&[&str]> for Packet {
    fn from(value: &[&str]) -> Self {
        Self::with_random_id(
            ServerData::ExecCommand,
            Vec::from(value).join(" ").as_bytes(),
        )
    }
}

impl From<&str> for Packet {
    fn from(value: &str) -> Self {
        Self::with_random_id(
            ServerData::Auth,
            &value.to_string().bytes().collect::<Vec<_>>(),
        )
    }
}

impl TryFrom<&Packet> for Vec<u8> {
    type Error = Error;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let mut payload = Self::new();
        payload.extend_from_slice(&packet.id.to_le_bytes());
        let type_id: i32 = packet.typ.into();
        payload.extend_from_slice(&type_id.to_le_bytes());
        payload.extend_from_slice(&packet.payload);
        payload.extend_from_slice(&TERMINATOR);

        if let Ok(size) = i32::try_from(payload.len()) {
            let mut bytes = Self::from(size.to_le_bytes());
            bytes.append(&mut payload);
            return Ok(bytes);
        }

        Err(Error::PacketSizeOverflow(payload.len()))
    }
}

impl TryFrom<&mut TcpStream> for Packet {
    type Error = Either<io::Error, Error>;

    fn try_from(stream: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut i32buf: [u8; 4] = [0, 0, 0, 0];
        stream.read_exact(&mut i32buf).map_err(Left)?;
        let size = i32::from_le_bytes(i32buf);

        match usize::try_from(size - 10) {
            Ok(payload_size) => {
                stream.read_exact(&mut i32buf).map_err(Left)?;
                let id = i32::from_le_bytes(i32buf);
                stream.read_exact(&mut i32buf).map_err(Left)?;
                let typ = ServerData::try_from(i32::from_le_bytes(i32buf)).map_err(Right)?;
                let mut payload = vec![0u8; payload_size];
                stream.read_exact(&mut payload).map_err(Left)?;
                let mut terminator: [u8; 2] = [0, 0];
                stream.read_exact(&mut terminator).map_err(Left)?;
                Ok(Self { id, typ, payload })
            }
            Err(_) => Err(Right(Error::PacketSizeUnderflow(size - 10))),
        }
    }
}
