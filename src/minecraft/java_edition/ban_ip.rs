//! Data structures related to IP banning.

use std::borrow::Cow;
use std::convert::Infallible;
use std::net::IpAddr;
use std::str::FromStr;

use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{Entity, Serialize};

/// A target for IP banning.
///
/// Can either be an IP address or an entity.
#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Ip(IpAddr),
    Entity(Entity<TargetSelector>),
}

impl From<IpAddr> for Target {
    fn from(ip: IpAddr) -> Target {
        Self::Ip(ip)
    }
}

impl From<Entity<TargetSelector>> for Target {
    fn from(entity: Entity<TargetSelector>) -> Target {
        Self::Entity(entity)
    }
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Target, Infallible> {
        if let Ok(ip) = IpAddr::from_str(s) {
            Ok(Self::Ip(ip))
        } else {
            Entity::from_str(s).map(Self::Entity)
        }
    }
}

impl Serialize for Target {
    fn serialize(self) -> Cow<'static, str> {
        match self {
            Self::Ip(ip) => Cow::Owned(ip.to_string()),
            Self::Entity(entity) => entity.serialize(),
        }
    }
}
