//! Client implementation for the Source `RCON` protocol.

mod client;
mod packet;
mod quirks;
mod server_data;
mod util;

use crate::RCon;
pub use client::Client;
pub use quirks::Quirks;

pub(crate) trait Source: RCon {}

impl Source for Client {}
