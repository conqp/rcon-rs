use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::hash::Hash;
use std::net::{IpAddr, SocketAddr};
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
        Self: RCon + Sized + Send,
        Self::Player: Debug,
    {
        async {
            self.players()
                .await
                .map(|players| PlayersMut::new(self, players))
        }
    }
}

/// Information about a player.
pub trait Player: Display {
    /// The type of ID the player is identified with.
    type Id: Clone + Debug + Display + Eq + Hash + Send;

    /// Returns the player's ID.
    ///
    /// Its return value shall be a value that can be used to securely identify the player.
    fn id(&self) -> Self::Id;

    /// The player's descriptive name.
    fn name(&self) -> Cow<'_, str>;

    /// Returns the index of the player if applicable.
    fn index(&self) -> Option<u64> {
        None
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
