//! Ban list related data structures and functions.

pub use entry::Entry;
use entry::NO_BANS;
pub use entry_type::EntryType;
pub use error::Error;

mod entry;
mod entry_type;
mod error;

/// Parse entries from a string.
pub(crate) fn parse_response(text: &str) -> Result<Vec<Entry>, Error> {
    let mut entries = Vec::new();

    if text.trim() == NO_BANS {
        return Ok(entries);
    }

    // Skip header "There are N ban(s):"
    for line in text.lines().skip(1) {
        entries.push(line.parse()?);
    }

    Ok(entries)
}
