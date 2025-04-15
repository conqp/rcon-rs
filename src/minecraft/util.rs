//! Utilities for the Minecraft extensions.

use crate::minecraft::Error;

const UNKNOWN_OR_INCOMPLETE_COMMAND: &str = "Unknown or incomplete command, see below for error";

pub fn parse_response(response: String) -> Result<String, Error> {
    if response.starts_with(UNKNOWN_OR_INCOMPLETE_COMMAND) {
        Err(Error::UnknownOrIncompleteCommand(response))
    } else {
        Ok(response)
    }
}
