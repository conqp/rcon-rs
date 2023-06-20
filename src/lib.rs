mod packet;
mod server_data;
use crate::packet::Packet;
use either::{Either, Left};
use std::io::{Error, Write};
use std::net::TcpStream;

pub fn rcon(host: &str, passwd: &str, command: &[&str]) -> Result<String, Either<Error, String>> {
    let mut stream = TcpStream::connect(host).map_err(Left)?;
    let login: Vec<u8> = Packet::from(passwd).into();
    stream.write(&login).map_err(Left)?;
    let _ = Packet::try_from(&mut stream)?;
    let command: Vec<u8> = Packet::from(command).into();
    stream.write(&command).map_err(Left)?;
    let command_response = Packet::try_from(&mut stream)?;
    Ok(command_response.text())
}
