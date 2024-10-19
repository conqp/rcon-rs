//! Client implementation of the `BattlEye Rcon` protocol.

mod client;
mod from_server;
mod header;
mod into_bytes;
mod packet;

use crate::RCon;
pub use client::Client;

/// A trait to identify `BattlEye Rcon` clients.
pub trait BattlEye: RCon {}

impl BattlEye for Client {}

impl<'a, T> BattlEye for &'a mut T where T: BattlEye {}
