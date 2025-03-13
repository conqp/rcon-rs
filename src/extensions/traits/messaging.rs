use std::future::Future;

/// Send direct messages to players.
pub trait Say {
    /// Send a message to a player identified by `target`.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn say(
        &mut self,
        target: &str,
        message: &str,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}

/// Broadcast messages to all players on the server.
pub trait Broadcast {
    /// Broadcast a message to all players on the server.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    fn broadcast(&mut self, message: &str) -> impl Future<Output = std::io::Result<()>> + Send;
}
