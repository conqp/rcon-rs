use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use uuid::Uuid;

/// Information about a player on a `DayZ` server.
#[derive(Clone, Debug)]
pub struct Player {
    id: i64,
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
        let guid: Uuid = fields
            .next()
            .ok_or("missing GUID")?
            .parse()
            .map_err(|error| format!("invalid GUID: {error}"))?;
        let name = fields.collect::<Vec<_>>().join("");
        Ok(Self {
            id,
            socket_addr,
            ping: Duration::from_millis(ping),
            guid,
            name,
        })
    }
}

impl crate::Player for Player {
    fn id(&self) -> i64 {
        self.id
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
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
