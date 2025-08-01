//! `RCON` extensions interface for Minecraft: Java Edition servers.

use std::borrow::Cow;
use std::future::Future;

pub use advancement::Grant;
pub use target_selector::{Argument, Sort, TargetSelector};

use crate::minecraft::{Entity, ResourceLocation, Serialize};
use crate::Minecraft;

pub mod advancement;
pub mod attribute;
pub mod ban;
pub mod ban_ip;
pub mod banlist;
pub mod bossbar;
pub mod target_selector;

/// Extension trait for `Source RCON` clients for Minecraft: Java Edition servers.
pub trait JavaEdition: Minecraft {
    /// Manage the target's ability.
    ///
    /// # Returns
    ///
    /// Returns an [`advancement::Proxy`] which can be used to execute
    /// advancement-related commands pertaining to the `target`.
    fn advancement(&mut self, target: Entity<TargetSelector>) -> advancement::Proxy<'_, Self>
    where
        Self: Sized,
    {
        advancement::Proxy::new(self, vec!["advancement".into(), target.serialize()])
    }

    /// Manage a target's attribute.
    ///
    /// # Returns
    ///
    /// Returns an [`attribute::Proxy`] which can be used to execute
    /// advancement-related commands pertaining to the `target`.
    fn attribute(
        &mut self,
        target: Entity<TargetSelector>,
        attribute: ResourceLocation,
    ) -> attribute::Proxy<'_, Self>
    where
        Self: Sized,
    {
        attribute::Proxy::new(
            self,
            vec![
                "attribute".into(),
                target.serialize(),
                attribute.serialize(),
            ],
        )
    }

    /// Adds player to banlist.
    ///
    /// # Errors
    ///
    /// Returns an [`ban::Error`] on errors.
    fn ban(
        &mut self,
        target: Entity<TargetSelector>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Option<ban::Entry>, ban::Error>> + Send;

    /// Adds IP address to banlist.
    ///
    /// Specifies the IP address to be added to the blacklist.
    /// Can also be a name of an online player, which represents the IP of that player.
    ///
    /// # Errors
    ///
    /// Returns an [`ban_ip::Error`] on errors.
    fn ban_ip<T>(
        &mut self,
        target: ban_ip::Target,
        reason: Option<T>,
    ) -> impl Future<Output = Result<(), ban_ip::Error>> + Send
    where
        T: AsRef<str> + Send;

    /// Return the entries from the ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`banlist::Error`] on errors.
    fn banlist(
        &mut self,
        entry_type: Option<banlist::EntryType>,
    ) -> impl Future<Output = Result<Vec<banlist::Entry>, banlist::Error>> + Send;

    /// Creates, modifies and lists bossbars.
    fn bossbar(&mut self) -> bossbar::Proxy<'_, Self>
    where
        Self: Sized,
    {
        bossbar::Proxy::new(self)
    }
}

impl<T> JavaEdition for T
where
    T: Minecraft + Send,
{
    /// Adds player to banlist.
    ///
    /// # Errors
    ///
    /// Returns an [`ban::Error`] on errors.
    async fn ban(
        &mut self,
        target: Entity<TargetSelector>,
        reason: Option<&str>,
    ) -> Result<Option<ban::Entry>, ban::Error> {
        let mut args = vec![Cow::Borrowed("ban"), target.serialize()];

        if let Some(reason) = reason {
            args.push(Cow::Borrowed(reason));
        }

        ban::parse_response(&self.run_utf8(args.join(" ")).await?)
    }

    /// Adds IP address to banlist.
    ///
    /// Specifies the IP address to be added to the blacklist.
    /// Can also be a name of an online player, which represents the IP of that player.
    ///
    /// # Errors
    ///
    /// Returns an [`ban_ip::Error`] on errors.
    async fn ban_ip<S>(
        &mut self,
        target: ban_ip::Target,
        reason: Option<S>,
    ) -> Result<(), ban_ip::Error>
    where
        S: AsRef<str> + Send,
    {
        let mut args = vec![Cow::Borrowed("ban_ip"), target.serialize()];

        if let Some(reason) = &reason {
            args.push(Cow::Borrowed(reason.as_ref()));
        }

        ban_ip::parse_response(&self.run_utf8(args.join(" ")).await?)
    }

    /// Return the entries from the ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`banlist::Error`] on errors.
    async fn banlist(
        &mut self,
        entry_type: Option<banlist::EntryType>,
    ) -> Result<Vec<banlist::Entry>, banlist::Error> {
        let mut args = vec![Cow::Borrowed("banlist")];

        if let Some(entry_type) = entry_type {
            args.push(entry_type.serialize());
        }

        banlist::parse_response(&self.run_utf8(args.join(" ")).await?)
    }
}
