//! Client implementation for the Source `RCON` protocol.

mod client;
mod packet;
mod quirks;
mod server_data;
mod util;

use crate::RCon;
pub use client::Client;
pub use quirks::Quirks;

/// A trait to identify `Source RCON` clients.
pub trait Source: RCon {}

impl Source for Client {}

impl<'a, T> Source for &'a mut T where T: Source {}
