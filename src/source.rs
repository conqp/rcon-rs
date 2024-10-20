//! Client implementation for the Source `RCON` protocol.

mod client;
mod packet;
mod quirks;
mod server_data;
mod util;

use crate::RCon;
pub use client::Client;
pub use quirks::Quirks;

/// Trait to identify `Source RCON` clients.
#[allow(private_bounds)]
pub trait Source: RCon {}

impl Source for Client {}
