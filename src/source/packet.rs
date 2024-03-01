use super::server_data::ServerData;
use super::util::invalid_data;
use futures::AsyncReadExt;
use log::debug;
use rand::{thread_rng, Rng};
use std::num::TryFromIntError;

const TERMINATOR: [u8; 2] = [0, 0];
const I32_BYTES: usize = 4;
const OFFSET: usize = 10;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
            ServerData::ExecCommand,
            command.to_vec(),
            TERMINATOR,
        )
    }

    pub async fn read_from(source: &mut async_std::net::TcpStream) -> std::io::Result<Self> {
        let mut buffer = [0; I32_BYTES];
        debug!("Reading payload size.");
        source.read_exact(&mut buffer).await?;
        let size: usize = i32::from_le_bytes(buffer)
            .try_into()
            .map_err(invalid_data)?;
        debug!("Packet size is {size}.");
        debug!("Reading packet ID.");
        source.read_exact(&mut buffer).await?;
        let id = i32::from_le_bytes(buffer);
        debug!("Packet ID is {id}.");
        debug!("Reading packet type.");
        source.read_exact(&mut buffer).await?;
        let typ: ServerData = i32::from_le_bytes(buffer)
            .try_into()
            .map_err(|value| invalid_data(format!("Invalid packet type: {value}")))?;
        debug!("Packet type is {typ:?}.");
        debug!("Reading payload.");
        let mut payload =
            vec![
                0;
                size.checked_sub(OFFSET)
                    .ok_or_else(|| invalid_data(format!("Invalid payload size: {size}")))?
            ];
        source.read_exact(&mut payload).await?;
        debug!("Packet payload is {payload:?}.");
        debug!("Reading terminator.");
        let mut terminator = [0; 2];
        source.read_exact(&mut terminator).await?;
        debug!("Packet terminator is {terminator:?}.");
        Ok(Self::new(id, typ, payload, terminator))
    }

    pub fn size(&self) -> usize {
        self.payload.len() + OFFSET
    }
}

impl TryFrom<Packet> for Vec<u8> {
    type Error = TryFromIntError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let mut bytes = Self::with_capacity(packet.size() + I32_BYTES);
        bytes.extend_from_slice(&i32::try_from(packet.size())?.to_le_bytes());
        bytes.extend_from_slice(&packet.id.to_le_bytes());
        bytes.extend_from_slice(&i32::from(packet.typ).to_le_bytes());
        bytes.extend_from_slice(&packet.payload);
        bytes.extend_from_slice(&packet.terminator);
        Ok(bytes)
    }
}

fn random_id<T>(mut rng: T) -> i32
where
    T: Rng,
{
    rng.gen_range(0..=i32::MAX)
}
