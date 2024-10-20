use std::borrow::Cow;
use std::future::Future;
use std::time::Duration;

use crate::Target;

/// Kick players from the server.
pub trait Kick {
    /// Kick a player from the server.
    ///
    /// You may specify an optional reason for the kick to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if kicking the player fails.
    fn kick(
        &mut self,
        player: Cow<'_, str>,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = std::io::Result<()>>;
}

/// Ban players from the server.
pub trait Ban {
    /// Ban a player from the server.
    ///
    /// You may specify an optional reason for the ban to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if banning  the player fails.
    fn ban(
        &mut self,
        player: Cow<'_, str>,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}

/// View the ban list of the server.
pub trait Bans {
    /// The ban list implementation to be returned.
    type BanListEntry: BanListEntry;

    /// Returns an iterator over the server's current ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if querying the ban list fails.
    fn bans(
        &mut self,
    ) -> impl Future<Output = std::io::Result<impl Iterator<Item = Self::BanListEntry>>> + Send;
}

/// An entry of a ban list.
pub trait BanListEntry {
    /// The unique ID of the entry.
    fn id(&self) -> u64;

    /// The target of the ban (IP address or UUID).
    fn target(&self) -> Target;

    /// The remaining duration of the ban.
    ///
    /// Returns `None` when this is a permanent ban.
    fn duration(&self) -> Option<Duration>;

    /// Returns the reason of the ban or `None` if no reason was specified.
    fn reason(&self) -> Option<&str>;
}

/// Ban list management.
pub trait AddBan {
    /// Add a ban to ban list.
    ///
    /// This can be either an IP address or a UUID.
    ///
    /// You may specify an optional duration and reason for the ban to add to the ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if banning  the player fails.
    fn add_ban(
        &mut self,
        ban: Target,
        duration: Option<Duration>,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}

/// Remove player bans.
pub trait RemoveBan {
    /// Remove a player ban entry from the server's ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if unbanning  the player fails.
    fn remove_ban(&mut self, id: u64) -> impl Future<Output = std::io::Result<()>> + Send;
}
