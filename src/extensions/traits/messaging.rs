use std::borrow::Cow;
use std::future::Future;

/// Send direct messages to players.
pub trait Say {
    /// Send a message to a player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn say<T>(
        &mut self,
        player: T,
        message: Cow<'_, str>,
    ) -> impl Future<Output = std::io::Result<()>> + Send
    where
        T: ToString + Send;
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
