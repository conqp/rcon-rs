use std::borrow::Cow;

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
