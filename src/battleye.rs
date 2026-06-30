//! Client implementation of the [`BattlEye Rcon`](https://www.battleye.com/downloads/BERConProtocol.txt) protocol.

pub use self::client::Client;
use crate::RCon;

mod client;
mod from_server;
mod header;
mod packet;

/// Trait to identify `BattlEye RCon` clients.
pub trait BattlEye: RCon {}

impl BattlEye for Client {}
