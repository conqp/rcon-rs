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
    pub fn new(id: i32, typ: ServerData, payload: &[u8]) -> Self {
        Self {
            id,
            typ,
            payload: payload.into(),
        }
    }

    pub fn with_random_id(typ: ServerData, payload: &[u8]) -> Self {
        Self::new(random(), typ, payload)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn typ(&self) -> ServerData {
        self.typ
    }

    pub fn text(&self) -> String {
        self.payload.iter().map(|byte| *byte as char).collect()
    }
}

impl AddAssign for Packet {
    fn add_assign(&mut self, rhs: Self) {
        self.typ = rhs.typ;
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

impl From<Packet> for Vec<u8> {
    fn from(packet: Packet) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&packet.id.to_le_bytes());
        let type_id: i32 = packet.typ.into();
        payload.extend_from_slice(&type_id.to_le_bytes());
        payload.extend_from_slice(&packet.payload);
        payload.extend_from_slice(&TERMINATOR);
        let size = payload.len() as i32;
        let mut bytes = Vec::from(size.to_le_bytes());
        bytes.append(&mut payload);
        bytes
    }
}

impl TryFrom<&mut &mut TcpStream> for Packet {
    type Error = Either<io::Error, Error>;

    fn try_from(stream: &mut &mut TcpStream) -> Result<Self, Self::Error> {
        let mut i32buf: [u8; 4] = [0, 0, 0, 0];
        stream.read_exact(&mut i32buf).map_err(Left)?;
        let size = i32::from_le_bytes(i32buf);
        stream.read_exact(&mut i32buf).map_err(Left)?;
        let id = i32::from_le_bytes(i32buf);
        stream.read_exact(&mut i32buf).map_err(Left)?;
        let typ = ServerData::try_from(i32::from_le_bytes(i32buf)).map_err(Right)?;
        let mut payload = vec![0u8; (size - 10) as usize];
        stream.read_exact(&mut payload).map_err(Left)?;
        let mut tail: [u8; 2] = [0, 0];
        stream.read_exact(&mut tail).map_err(Left)?;
        Ok(Self { id, typ, payload })
    }
}
