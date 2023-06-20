mod packet;
mod server_data;
use crate::packet::Packet;
use either::{Either, Left};
use std::io::{Error, Write};
use std::net::TcpStream;

pub fn rcon(
    host: &str,
    passwd: &str,
    command: &[&str],
    fragmentation_threshold: Option<usize>,
) -> Result<String, Either<Error, String>> {
    let mut stream = TcpStream::connect(host).map_err(Left)?;
    communicate(&mut stream, Packet::from(passwd), fragmentation_threshold)?;
    let command_response =
        communicate(&mut stream, Packet::from(command), fragmentation_threshold)?;
    Ok(command_response.text())
}

pub fn communicate(
    mut stream: &mut TcpStream,
    packet: Packet,
    fragmentation_threshold: Option<usize>,
) -> Result<Packet, Either<Error, String>> {
    let bytes: Vec<u8> = packet.into();
    stream.write(bytes.as_slice()).map_err(Left)?;
    let mut response = Packet::try_from(&mut stream)?;

    if response.text().len() < fragmentation_threshold.unwrap_or(4096) {
        return Ok(response);
    }

    while let Ok(packet) = Packet::try_from(&mut stream) {
        response += packet;
        eprint!("Added packet");
    }

    Ok(response)
}
