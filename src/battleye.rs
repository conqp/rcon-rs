//! Client implementation of the `BattlEye Rcon` protocol.

mod client;
mod from_server;
mod header;
mod packet;
mod to_server;

pub use client::Client;
