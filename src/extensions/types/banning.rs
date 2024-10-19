use std::net::IpAddr;
use std::str::FromStr;
use uuid::Uuid;

/// The target of a ban.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Target {
    /// Ban of an IP address.
    Ip(IpAddr),
    /// Ban of a UUID.
    Uuid(Uuid),
}

impl FromStr for Target {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IpAddr::from_str(s).map_or_else(
            |_| Uuid::from_str(s).map_or(Err(()), |uuid| Ok(Self::Uuid(uuid))),
            |ip| Ok(Self::Ip(ip)),
        )
    }
}
