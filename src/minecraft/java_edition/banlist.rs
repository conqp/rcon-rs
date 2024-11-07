//! Ban list related data structures and functions.

pub use entry::Entry;
use entry::NO_BANS;
pub use entry_type::EntryType;

mod entry;
mod entry_type;

/// Parse entries from a string.
pub(crate) fn parse_entries(text: &str) -> Result<Vec<Entry>, ()> {
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
