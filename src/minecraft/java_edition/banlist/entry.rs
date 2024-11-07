use std::str::FromStr;

use regex::Regex;

use super::error::Error;
use crate::minecraft::java_edition::ban_ip::Target;

pub(crate) const NO_BANS: &str = "There are no bans";
const REGEX: &str = r"(.+) was banned by (.+): (.+)";

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    target: Target,
    moderator: String,
    reason: String,
}

impl Entry {
    /// The ban target.
    pub const fn target(&self) -> &Target {
        &self.target
    }

    /// The name of the moderator who issued the ban.
    pub fn moderator(&self) -> &str {
        &self.moderator
    }

    /// The reason for the ban.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl FromStr for Entry {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let regex =
            Regex::new(REGEX).expect("Ban list entry regex should be valid. This is a bug.");

        let Some((_, [target, moderator, reason])) = regex
            .captures(text.trim())
            .map(|captures| captures.extract())
        else {
            return Err(Error::InvalidEntry(text.to_string()));
        };

        Ok(Self {
            target: Target::from_str(target)
                .expect("Target parsing should be infallible. This is a bug."),
            moderator: moderator.to_string(),
            reason: reason.to_string(),
        })
    }
}
