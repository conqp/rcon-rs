//! Utilities for the Minecraft extensions.

use std::io::{Error, ErrorKind};

const UNKNOWN_OR_INCOMPLETE_COMMAND: &str = "Unknown or incomplete command, see below for error";

pub fn parse_response(response: String) -> std::io::Result<String> {
    if response.starts_with(UNKNOWN_OR_INCOMPLETE_COMMAND) {
        Err(Error::new(ErrorKind::Other, response))
    } else {
        Ok(response)
    }
}
