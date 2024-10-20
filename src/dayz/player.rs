use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use uuid::Uuid;

/// Information about a player on a `DayZ` server.
#[derive(Clone, Debug)]
pub struct Player {
    index: u64,
    socket_addr: SocketAddr,
    ping: Duration,
    guid: Uuid,
    name: String,
}

impl Player {
    /// The index of the player on the server.
    #[must_use]
    pub const fn index(&self) -> u64 {
        self.index
    }

    /// The socket address from which the player connected.
    #[must_use]
    pub const fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }

    /// The player's round trip time (RTT).
    #[must_use]
    pub const fn ping(&self) -> Duration {
        self.ping
    }

    /// The player's globally unique identifier.
    #[must_use]
    pub const fn guid(&self) -> Uuid {
        self.guid
    }

    /// The player's name.
    #[must_use]
    pub fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"#{} {} alias "{}" from {} with RTT of {}ms"#,
            self.index,
            self.guid,
            self.name,
            self.socket_addr,
            self.ping.as_millis()
        )
    }
}

impl FromStr for Player {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split_whitespace();
        let id = fields
            .next()
            .ok_or("missing ID field")?
            .parse()
            .map_err(|error| format!("invalid ID: {error}"))?;
        let socket_addr = fields
            .next()
            .ok_or("missing socket address")?
            .parse()
            .map_err(|error| format!("invalid socket address: {error}"))?;
        let ping: u64 = fields
            .next()
            .ok_or("missing ping")?
            .parse()
            .map_err(|error| format!("invalid ping: {error}"))?;
        let guid = fields.next().ok_or("missing GUID")?;
        let guid = guid
            .split_once('(')
            .map_or(guid, |(guid, _)| guid)
            .parse()
            .map_err(|error| format!("invalid GUID: {error}"))?;
        let name = fields.collect::<Vec<_>>().join("");
        Ok(Self {
            index: id,
            socket_addr,
            ping: Duration::from_millis(ping),
            guid,
            name,
        })
    }
}
