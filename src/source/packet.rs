use super::server_data::ServerData;
use rand::{thread_rng, Rng};
use std::io;
use std::io::Read;

const TERMINATOR: [u8; 2] = [0, 0];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub(crate) id: i32,
    pub(crate) typ: ServerData,
    pub(crate) payload: Vec<u8>,
    pub(crate) terminator: [u8; 2],
}

impl Packet {
    pub const fn new(id: i32, typ: ServerData, payload: Vec<u8>, terminator: [u8; 2]) -> Self {
        Self {
            id,
            typ,
            payload,
            terminator,
        }
    }

    pub fn login(password: &str) -> Self {
        Self::new(
            random_id(thread_rng()),
            ServerData::Auth,
            password.as_bytes().to_vec(),
            TERMINATOR,
        )
    }

    pub fn command<T>(args: &[T]) -> Self
    where
        T: AsRef<str>,
    {
        Self::command_str(
            args.iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>()
                .join(" ")
                .as_str(),
        )
    }

    pub fn command_str(command: &str) -> Self {
        Self::command_raw(command.as_bytes())
    }

    pub fn command_raw(command: &[u8]) -> Self {
        Self::new(
            random_id(thread_rng()),
            ServerData::Auth,
            command.to_vec(),
            TERMINATOR,
        )
    }

    pub fn read_from<T>(mut source: T) -> io::Result<Self>
    where
        T: Read,
    {
        let mut buffer = [0; 4];
        source.read_exact(&mut buffer)?;
        let size: usize = i32::from_le_bytes(buffer)
            .try_into()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        source.read_exact(&mut buffer)?;
        let id = i32::from_le_bytes(buffer);
        source.read_exact(&mut buffer)?;
        let typ: ServerData = i32::from_le_bytes(buffer).try_into().map_err(|value| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid packet type: {value}"),
            )
        })?;
        let mut payload = vec![0; size];
        source.read_exact(&mut payload)?;
        let mut terminator = [0; 2];
        source.read_exact(&mut terminator)?;
        Ok(Self::new(id, typ, payload, terminator))
    }
}

impl From<Packet> for Vec<u8> {
    fn from(packet: Packet) -> Self {
        let mut bytes = Self::with_capacity(4 + 4 + packet.payload.len() + 2);
        bytes.extend_from_slice(&packet.id.to_le_bytes());
        bytes.extend_from_slice(&i32::from(packet.typ).to_le_bytes());
        bytes.extend_from_slice(&packet.payload);
        bytes.extend_from_slice(&packet.terminator);
        bytes
    }
}

fn random_id<T>(mut rng: T) -> i32
where
    T: Rng,
{
    rng.gen_range(0..=i32::MAX)
}
