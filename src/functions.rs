use crate::packet::Packet;
use crate::Error;
use either::{Either, Left, Right};
use std::io;
use std::io::Write;
use std::net::TcpStream;

/// Executes a command on the server
/// # Errors
/// Returns `Either<io::Error, rcon_rs::Error>` on either I/O or protocol errors
pub fn rcon(
    host: &str,
    passwd: &str,
    command: &[&str],
    fragmentation_threshold: Option<usize>,
) -> Result<String, Either<io::Error, Error>> {
    let mut stream = TcpStream::connect(host).map_err(Left)?;
    communicate(&mut stream, &Packet::from(passwd), fragmentation_threshold)?;
    let command_response =
        communicate(&mut stream, &Packet::from(command), fragmentation_threshold)?;
    Ok(command_response.text())
}

/// Exchanges a packet with the server
/// # Errors
/// Returns `Either<io::Error, rcon_rs::Error>` on either I/O or protocol errors
pub fn communicate(
    stream: &mut TcpStream,
    packet: &Packet,
    fragmentation_threshold: Option<usize>,
) -> Result<Packet, Either<io::Error, Error>> {
    let fragmentation_threshold = fragmentation_threshold.unwrap_or(4096);

    let bytes: Vec<u8> = packet.try_into().map_err(Right)?;
    stream.write(bytes.as_slice()).map_err(Left)?;
    let mut response = Packet::try_from(&mut *stream)?;

    if response.text().len() < fragmentation_threshold {
        return Ok(response);
    }

    while let Ok(packet) = Packet::try_from(&mut *stream) {
        let size = packet.text().len();
        response += packet;

        if size < fragmentation_threshold {
            return Ok(response);
        }
    }

    Ok(response)
}
