use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

pub use target::Target;

mod target;

pub const PERM_BAN: &str = "perm";
pub const SECS_PER_MINUTE: u64 = 60;

/// A ban list entry.
#[derive(Debug)]
pub struct BanListEntry {
    index: u64,
    target: Target,
    duration: Option<Duration>,
    reason: Option<String>,
}

impl BanListEntry {
    /// The index of the ban list entry.
    #[must_use]
    pub const fn index(&self) -> u64 {
        self.index
    }

    /// The target that was banned.
    ///
    /// This may either be an IP address or a UUID.
    #[must_use]
    pub const fn target(&self) -> Target {
        self.target
    }

    /// The duration of the ban.
    ///
    /// A value of `None` indicates a permanent ban.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Returns the reason for the ban.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

impl Display for BanListEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} of ", self.index)?;

        match self.target {
            Target::Ip(ip) => Display::fmt(&ip, f)?,
            Target::Uuid(uuid) => Display::fmt(&uuid, f)?,
        }

        if let Some(duration) = self.duration {
            write!(f, " for {} more seconds", duration.as_secs())?;
        } else {
            write!(f, " forever")?;
        }

        if let Some(reason) = &self.reason {
            write!(f, r#" because of "{reason}""#)?;
        }

        Ok(())
    }
}

impl FromStr for BanListEntry {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split_whitespace();
        let id = fields.next().ok_or("Missing ID field")?;
        let id: u64 = id
            .parse()
            .map_err(|_| format!("Invalid u64 for ID: {id}"))?;
        let target = fields.next().ok_or("Missing ban target field")?;
        let target =
            Target::from_str(target).map_err(|()| format!("Invalid ban type: {target}"))?;
        let duration = fields.next().ok_or("Missing duration field")?;
        let duration = if duration == PERM_BAN {
            None
        } else {
            u64::from_str(duration)
                .map(|minutes| Some(Duration::from_secs(minutes * SECS_PER_MINUTE)))
                .map_err(|_| format!("Invalid duration: {duration}"))?
        };
        let reason = fields.next().map(ToString::to_string);

        Ok(Self {
            index: id,
            target,
            duration,
            reason,
        })
    }
}
