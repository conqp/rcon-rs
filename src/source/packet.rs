use log::{debug, trace, warn};
use num_traits::FromPrimitive;
use rand::{rng, Rng};
use std::num::TryFromIntError;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use super::quirks::Quirks;
use super::server_data::ServerData;
use super::util::invalid_data;

const TERMINATOR: [u8; 2] = [0, 0];
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
            random_id(rng()),
            ServerData::Auth,
            password.bytes().collect(),
            TERMINATOR,
        )
    }

    pub fn command(command: &[u8]) -> Self {
        Self::new(
            random_id(rng()),
            ServerData::ExecCommandOrAuthResponse,
            command.to_vec(),
            TERMINATOR,
        )
    }

    pub const fn sentinel(id: i32) -> Self {
        Self::new(
            id.wrapping_add(1),
            ServerData::ResponseValue,
            Vec::new(),
            TERMINATOR,
        )
    }

    pub async fn read_from(source: &mut TcpStream) -> std::io::Result<Self> {
        let mut buffer = [0; size_of::<i32>()];
        debug!("Reading payload size.");
        source.read_exact(&mut buffer).await?;
        let size: usize = i32::from_le_bytes(buffer)
            .try_into()
            .map_err(invalid_data)?;
        trace!("Packet size is {size}.");

        debug!("Reading packet ID.");
        source.read_exact(&mut buffer).await?;
        let id = i32::from_le_bytes(buffer);
        trace!("Packet ID is {id}.");

        debug!("Reading packet type.");
        source.read_exact(&mut buffer).await?;
        let type_id = i32::from_le_bytes(buffer);
        let typ = ServerData::from_i32(type_id)
            .ok_or_else(|| invalid_data(format!("Invalid packet type: {type_id:#010X}")))?;
        trace!("Packet type is {typ:?}.");

        debug!("Reading payload.");
        let mut payload =
            vec![
                0;
                size.checked_sub(OFFSET)
                    .ok_or_else(|| invalid_data(format!("Invalid payload size: {size}")))?
            ];
        source.read_exact(&mut payload).await?;
        trace!("Packet payload is {payload:?}.");

        debug!("Reading terminator.");
        let mut terminator = [0; 2];
        source.read_exact(&mut terminator).await?;
        trace!("Packet terminator is {terminator:?}.");

        if terminator != TERMINATOR {
            warn!("Received non-standard terminator: {terminator:?}");
        }

        Ok(Self::new(id, typ, payload, terminator))
    }

    pub fn size(&self) -> usize {
        self.payload.len() + OFFSET
    }

    pub fn validate(&self, id: i32, quirks: Quirks) -> std::io::Result<()> {
        if self.id == id {
            Ok(())
        } else if self.id == 0x00 && quirks.contains(Quirks::PALWORLD) {
            debug!("Packet ID does not match, but accepting packet due to Palworld quirk.");
            Ok(())
        } else {
            Err(invalid_data(format!(
                "Packet ID mismatch: {} != {id}",
                self.id
            )))
        }
    }
}

impl TryFrom<Packet> for Vec<u8> {
    type Error = TryFromIntError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let mut bytes = Self::with_capacity(packet.size() + size_of::<i32>());
        bytes.extend_from_slice(&i32::try_from(packet.size())?.to_le_bytes());
        bytes.extend_from_slice(&packet.id.to_le_bytes());
        bytes.extend_from_slice(&(packet.typ as i32).to_le_bytes());
        bytes.extend_from_slice(&packet.payload);
        bytes.extend_from_slice(&packet.terminator);
        Ok(bytes)
    }
}

fn random_id<T>(mut rng: T) -> i32
where
    T: Rng,
{
    rng.random_range(0..=i32::MAX)
}
