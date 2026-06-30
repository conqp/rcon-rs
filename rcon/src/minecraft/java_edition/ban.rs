//! Datastructures and functions related to banning.

use std::str::FromStr;

pub use entry::Entry;
pub use error::Error;

mod entry;
mod error;

const ALREADY_BANNED: &str = "Nothing changed. The player is already banned";
const UNKNOWN_SELECTOR_TYPE: &str = "Unknown selector type ";

pub(crate) fn parse_response(text: &str) -> Result<Option<Entry>, Error> {
    if text.starts_with(UNKNOWN_SELECTOR_TYPE) {
        return Err(Error::UnknownSelector);
    }

    if text.trim() == ALREADY_BANNED {
        return Ok(None);
    }

    Entry::from_str(text).map(Some)
}
