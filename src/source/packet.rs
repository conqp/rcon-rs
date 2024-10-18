use std::borrow::Cow;
use std::io::Read;
use std::net::TcpStream;
use std::num::TryFromIntError;

use log::{debug, trace, warn};
use rand::{thread_rng, Rng};

use super::server_data::ServerData;
use super::util::invalid_data;

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

    pub fn login<'a, T>(password: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self::new(
            random_id(thread_rng()),
            ServerData::Auth,
            password.into().bytes().collect(),
            TERMINATOR,
        )
    }

    pub fn command(args: &[Cow<'_, str>]) -> Self {
        Self::command_raw(args.join(" ").as_bytes())
    }

    pub fn command_raw(command: &[u8]) -> Self {
        Self::new(
            random_id(thread_rng()),
            ServerData::ExecCommand,
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

    pub fn read_from(source: &mut TcpStream) -> std::io::Result<Self> {
        let mut buffer = [0; I32_BYTES];
        debug!("Reading payload size.");
        source.read_exact(&mut buffer)?;
        let size: usize = i32::from_le_bytes(buffer)
            .try_into()
            .map_err(invalid_data)?;
        trace!("Packet size is {size}.");

        debug!("Reading packet ID.");
        source.read_exact(&mut buffer)?;
        let id = i32::from_le_bytes(buffer);
        trace!("Packet ID is {id}.");

        debug!("Reading packet type.");
        source.read_exact(&mut buffer)?;
        let typ: ServerData = i32::from_le_bytes(buffer)
            .try_into()
            .map_err(|value| invalid_data(format!("Invalid packet type: {value}")))?;
        trace!("Packet type is {typ:?}.");

        debug!("Reading payload.");
        let mut payload =
            vec![
                0;
                size.checked_sub(OFFSET)
                    .ok_or_else(|| invalid_data(format!("Invalid payload size: {size}")))?
            ];
        source.read_exact(&mut payload)?;
        trace!("Packet payload is {payload:?}.");

        debug!("Reading terminator.");
        let mut terminator = [0; 2];
        source.read_exact(&mut terminator)?;
        trace!("Packet terminator is {terminator:?}.");

        if terminator != TERMINATOR {
            warn!("Received non-standard terminator: {terminator:?}");
        }

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
