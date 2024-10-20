use std::borrow::Cow;
use std::fmt::Debug;
use std::future::Future;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use uuid::Uuid;

use crate::{PlayersMut, RCon};

/// Manage players on the server.
pub trait Players {
    /// The player type that is being returned.
    type Player: Player;

    /// List players on the server.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if listing the players fails.
    fn players(&mut self) -> impl Future<Output = std::io::Result<Vec<Self::Player>>> + Send;

    /// Returns an iterator over player proxies.
    ///
    /// Each player proxy also implements [`Player`] but also some other functionalities,
    /// depending on the traits that the underlying RCON client implements.
    ///
    /// This can be used to call methods on players directly, such as messaging,
    /// kicking and banning.
    ///
    /// Note that this method does not make any guarantees on the validity of the player information,
    /// which may or may not change, while the proxy object is held, so use this with care.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if listing the players fails.
    fn players_mut(&mut self) -> impl Future<Output = std::io::Result<PlayersMut<'_, Self>>> + Send
    where
        Self: RCon + Sized,
    {
        async {
            self.players()
                .await
                .map(|players| PlayersMut::new(self, players))
        }
    }
}

/// Information about a player.
pub trait Player: Debug + Send {
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
