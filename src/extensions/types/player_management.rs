use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use std::vec::IntoIter;

use uuid::Uuid;

use crate::{Ban, Kick, Player, Players, RCon, Say};

/// An iterator over proxy objects of players on the server.
#[derive(Debug)]
pub struct PlayersMut<'client, C>
where
    C: RCon + Players,
    <C as Players>::Player: Debug,
{
    client: &'client mut C,
    players: IntoIter<<C as Players>::Player>,
}

impl<'client, C> PlayersMut<'client, C>
where
    C: RCon + Players,
    <C as Players>::Player: Debug,
{
    pub(crate) fn new(client: &'client mut C, players: Vec<<C as Players>::Player>) -> Self {
        Self {
            client,
            players: players.into_iter(),
        }
    }

    /// Returns the next player proxy from the player list iterator.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<PlayerProxy<'_, C, <C as Players>::Player>> {
        self.players
            .next()
            .map(|player| PlayerProxy::new(&mut *self.client, player))
    }
}

/// A proxy type to act on a player.
#[derive(Debug)]
pub struct PlayerProxy<'client, C, P>
where
    C: RCon,
    P: Player,
{
    client: &'client mut C,
    player: P,
}

impl<'client, C, P> PlayerProxy<'client, C, P>
where
    C: RCon,
    P: Player,
{
    pub(crate) fn new(client: &'client mut C, player: P) -> Self {
        Self { client, player }
    }

    /// Send a message to this player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    pub async fn say(&mut self, message: Cow<'_, str>) -> std::io::Result<()>
    where
        C: Say + Send,
        P: Send,
    {
        Say::say(self.client, self.player.id(), message).await
    }

    /// Kick this player from the server.
    ///
    /// You may specify an optional reason for the kick to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if kicking the player fails.
    pub async fn kick(&mut self, reason: Option<Cow<'_, str>>) -> std::io::Result<()>
    where
        C: Kick + Send,
        P: Send,
    {
        Kick::kick(self.client, self.player.id(), reason).await
    }

    /// Ban this player from the server.
    ///
    /// You may specify an optional reason for the ban to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if banning  the player fails.
    pub async fn ban(&mut self, reason: Option<Cow<'_, str>>) -> std::io::Result<()>
    where
        C: Ban + Send,
        P: Send,
    {
        Ban::ban(self.client, self.player.id(), reason).await
    }
}

impl<'client, C, P> Display for PlayerProxy<'client, C, P>
where
    C: RCon,
    P: Player,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.player, f)
    }
}

impl<'client, C, P> Player for PlayerProxy<'client, C, P>
where
    C: RCon,
    P: Player,
{
    type Id = P::Id;

    fn id(&self) -> Self::Id {
        self.player.id()
    }

    fn name(&self) -> Cow<'_, str> {
        self.player.name()
    }

    fn index(&self) -> Option<u64> {
        self.player.index()
    }

    fn uuid(&self) -> Option<Uuid> {
        self.player.uuid()
    }

    fn socket_addr(&self) -> Option<SocketAddr> {
        self.player.socket_addr()
    }

    fn ip_add(&self) -> Option<IpAddr> {
        self.player.ip_add()
    }

    fn rtt(&self) -> Option<Duration> {
        self.player.rtt()
    }
}
