use crate::server_data::ServerData;
use either::{Either, Left, Right};
use rand::random;
use std::io::{Error, Read};
use std::net::TcpStream;

const TERMINATOR: [u8; 2] = [0, 0];

#[derive(Debug, Eq, PartialEq)]
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

    pub fn text(&self) -> String {
        self.payload.iter().map(|byte| *byte as char).collect()
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

impl TryFrom<&mut TcpStream> for Packet {
    type Error = Either<Error, String>;

    fn try_from(stream: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        stream.read_exact(&mut buf).map_err(Left)?;
        let size = i32::from_le_bytes(buf);
        stream.read_exact(&mut buf).map_err(Left)?;
        let id = i32::from_le_bytes(buf);
        stream.read_exact(&mut buf).map_err(Left)?;
        let typ = ServerData::try_from(i32::from_le_bytes(buf)).map_err(Right)?;
        let mut sep: [u8; 2] = [0, 0];
        stream.read_exact(&mut sep).map_err(Left)?;
        let mut payload = vec![0u8; (size - 10) as usize];
        stream.read_exact(&mut payload).map_err(Left)?;
        Ok(Self { id, typ, payload })
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
