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

impl crate::Player for Player {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.guid
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn index(&self) -> Option<u64> {
        Some(self.index)
    }

    fn uuid(&self) -> Option<Uuid> {
        Some(self.guid)
    }

    fn socket_addr(&self) -> Option<SocketAddr> {
        Some(self.socket_addr)
    }

    fn rtt(&self) -> Option<Duration> {
        Some(self.ping)
    }
}
