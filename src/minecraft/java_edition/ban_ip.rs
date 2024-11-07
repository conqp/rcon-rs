//! Data structures related to IP banning.

pub use error::Error;
pub use target::Target;

mod error;
mod target;

const UNKNOWN_PLAYER: &str = "Invalid IP address or unknown player";

pub(crate) fn parse_response(text: &str) -> Result<(), Error> {
    if text.trim() == UNKNOWN_PLAYER {
        Err(Error::UnknownPlayer)
    } else {
        Ok(())
    }
}
