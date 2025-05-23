//! Client implementation of the [`BattlEye Rcon`](https://www.battleye.com/downloads/BERConProtocol.txt) protocol.

mod client;
mod from_server;
mod header;
mod into_bytes;
mod packet;

use crate::RCon;
pub use client::Client;

/// Trait to identify `BattlEye RCon` clients.
pub trait BattlEye: RCon {}

impl BattlEye for Client {}
