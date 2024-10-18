//! Client implementation of the `BattlEye Rcon` protocol.

mod client;
mod from_server;
mod header;
mod into_bytes;
mod packet;

pub use client::Client;
