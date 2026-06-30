//! Client implementation for the [`Source RCON`](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol) protocol.

pub use self::client::Client;
pub use self::quirks::Quirks;
use crate::RCon;

mod client;
mod packet;
mod quirks;
mod server_data;
mod util;

/// Trait to identify `Source RCON` clients.
pub trait Source: RCon {}

impl Source for Client {}
