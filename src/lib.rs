mod packet;
mod server_data;
use crate::packet::Packet;
use either::{Either, Left};
use std::io::{Error, Write};
use std::net::TcpStream;

pub fn rcon(host: &str, passwd: &str, command: &[&str]) -> Result<String, Either<Error, String>> {
    let mut stream = TcpStream::connect(host).map_err(Left)?;
    communicate(&mut stream, Packet::from(passwd))?;
    let command_response = communicate(&mut stream, Packet::from(command))?;
    Ok(command_response.text())
}

pub fn communicate(
    stream: &mut TcpStream,
    packet: Packet,
) -> Result<Packet, Either<Error, String>> {
    let bytes: Vec<u8> = packet.into();
    stream.write(bytes.as_slice()).map_err(Left)?;
    Packet::try_from(stream)
}
