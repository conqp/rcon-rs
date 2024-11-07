use std::str::FromStr;

use regex::Regex;

use crate::minecraft::java_edition::ban::Error;
use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::Entity;

const REGEX: &str = r"Banned (.+): (.+)";

/// A new ban entry containing the name of the banned target and the reason for the ban.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    target: Entity<TargetSelector>,
    reason: String,
}

impl Entry {
    pub(crate) const fn new(target: Entity<TargetSelector>, reason: String) -> Self {
        Self { target, reason }
    }

    /// The banned target.
    #[must_use]
    pub const fn target(&self) -> &Entity<TargetSelector> {
        &self.target
    }

    /// The reason for the ban.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl FromStr for Entry {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(REGEX).expect("The ban regex should be valid. This is a bug.");

        let Some((_, [target, reason])) = regex
            .captures(text.trim())
            .map(|captures| captures.extract())
        else {
            return Err(Error::Other(text.to_string()));
        };

        Ok(Self {
            target: target
                .parse()
                .expect("Parsing a ban target should be infallible. This is a bug."),
            reason: reason.to_string(),
        })
    }
}
