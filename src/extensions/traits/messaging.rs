use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::hash::Hash;

/// Send direct messages to players.
pub trait Say {
    /// The type of ID the player is identified with.
    type Id: Clone + Debug + Display + Eq + Hash;

    /// Send a message to a player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn say(
        &mut self,
        player: Self::Id,
        message: Cow<'_, str>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}

/// Broadcast messages to all players on the server.
pub trait Broadcast {
    /// Broadcast a message to all players on the server.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn broadcast(
        &mut self,
        message: Cow<'_, str>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}
