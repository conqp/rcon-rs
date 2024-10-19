use std::borrow::Cow;
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
{
    client: &'client mut C,
    players: IntoIter<<C as Players>::Player>,
}

impl<'client, C> PlayersMut<'client, C>
where
    C: RCon + Players,
{
    pub(crate) fn new(client: &'client mut C, players: Vec<<C as Players>::Player>) -> Self {
        Self {
            client,
            players: players.into_iter(),
        }
    }

    /// Returns the next player proxy from the player list iterator.
    pub fn next<'borrow>(
        &'borrow mut self,
    ) -> Option<PlayerProxy<&'borrow mut C, <C as Players>::Player>>
    where
        &'borrow mut C: RCon,
    {
        self.players
            .next()
            .map(|player| PlayerProxy::new(&mut *self.client, player))
    }
}

/// A proxy type to act on a player.
#[derive(Debug)]
pub struct PlayerProxy<C, P>
where
    C: RCon,
    P: Player,
{
    client: C,
    player: P,
}

impl<C, P> PlayerProxy<C, P>
where
    C: RCon,
    P: Player,
{
    pub(crate) const fn new(client: C, player: P) -> Self {
        Self { client, player }
    }

    /// Send a message to this player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if sending the message fails.
    pub fn say(&mut self, message: Cow<'_, str>) -> std::io::Result<()>
    where
        C: Say,
    {
        Say::say(&mut self.client, self.player.id(), message)
    }

    /// Kick this player from the server.
    ///
    /// You may specify an optional reason for the kick to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if kicking the player fails.
    pub fn kick(&mut self, reason: Option<Cow<'_, str>>) -> std::io::Result<()>
    where
        C: Kick,
    {
        Kick::kick(&mut self.client, self.player.id(), reason)
    }

    /// Ban this player from the server.
    ///
    /// You may specify an optional reason for the ban to forward to the player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if banning  the player fails.
    fn ban(&mut self, player: Cow<'_, str>, reason: Option<Cow<'_, str>>) -> std::io::Result<()>
    where
        C: Ban,
    {
        Ban::ban(&mut self.client, player, reason)
    }
}

impl<C, P> Player for PlayerProxy<C, P>
where
    C: RCon,
    P: Player,
{
    fn id(&self) -> Cow<'_, str> {
        self.player.id()
    }

    fn numeric_id(&self) -> Option<i64> {
        self.player.numeric_id()
    }

    fn name(&self) -> Cow<'_, str> {
        self.player.name()
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
