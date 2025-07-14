//! Client implementation of the [`BattlEye Rcon`](https://www.battleye.com/downloads/BERConProtocol.txt) protocol.

mod client;
mod from_server;
mod header;
mod into_bytes;
mod packet;

pub use client::Client;

use crate::RCon;

/// Trait to identify `BattlEye RCon` clients.
pub trait BattlEye: RCon {}

impl BattlEye for Client {}
