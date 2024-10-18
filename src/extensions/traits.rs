use std::borrow::Cow;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use uuid::Uuid;

/// Send direct messages to players.
pub trait Say {
    /// Send a message to a player identified by `target`.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn say(&mut self, target: Cow<'_, str>, message: Cow<'_, str>) -> std::io::Result<()>;
}

/// Broadcast messages to all players on the server.
pub trait Broadcast {
    /// Broadcast a message to all players on the server.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn broadcast(&mut self, message: Cow<'_, str>) -> std::io::Result<()>;
}

/// Kick a player from the server.
pub trait Kick {
    /// Kick a player from the server.
    ///
    /// You may specify an optional reason for the kick to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if kicking the player fails.
    fn kick(&mut self, player: Cow<'_, str>, reason: Option<Cow<'_, str>>) -> std::io::Result<()>;
}

/// Kick a player from the server.
pub trait Ban {
    /// Kick a player from the server.
    ///
    /// You may specify an optional reason for the ban to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if kicking the player fails.
    fn ban(&mut self, player: Cow<'_, str>, reason: Option<Cow<'_, str>>) -> std::io::Result<()>;
}

/// List players on the server.
pub trait Players {
    /// The player type that is being returned.
    type Player: Player;

    /// List players on the server.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if listing the players fails.
    fn players(&mut self) -> std::io::Result<Vec<Self::Player>>;
}

/// Information about a player.
pub trait Player {
    /// Returns the player's ID.
    ///
    /// This is the only mandatory method of `Player` and may return
    /// the player's name or the string representation of a numeric ID.
    ///
    /// Its return value shall be a value that can be used to securely identify the player.
    fn id(&self) -> Cow<'_, str>;

    /// Returns the player's ID.
    fn numeric_id(&self) -> Option<i64> {
        i64::from_str(self.name().as_ref()).ok()
    }

    /// The player's descriptive name.
    ///
    /// This defaults to the return value of [`id`](Self::id) but unlike the
    /// latter this method should not be used to safely identify players.
    fn name(&self) -> Cow<'_, str> {
        self.id()
    }

    /// Returns the player's UUID.
    fn uuid(&self) -> Option<Uuid> {
        None
    }

    /// Returns the player's socket address.
    fn socket_addr(&self) -> Option<SocketAddr> {
        None
    }

    /// Returns the player's IP address.
    fn ip_add(&self) -> Option<IpAddr> {
        self.socket_addr().map(|addr| addr.ip())
    }

    /// Returns the player's RTT (aka "ping").
    fn rtt(&self) -> Option<Duration> {
        None
    }
}
