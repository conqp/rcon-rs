//! `RCON` extensions interface for Minecraft: Java Edition servers.

use std::borrow::Cow;
use std::future::Future;

use crate::minecraft::{Entity, ResourceLocation, Serialize};
use crate::Minecraft;

pub use advancement::Grant;
pub use target_selector::{Argument, Sort, TargetSelector};

pub mod advancement;
pub mod attribute;
pub mod ban_ip;
pub mod banlist;
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
        Self: Sized + Send,
    {
        advancement::Proxy::new(self, target)
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
        Self: Sized + Send,
    {
        attribute::Proxy::new(self, target, attribute)
    }

    /// Adds player to banlist.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occur
    /// or if the returned bytes are not valid UTF-8.
    fn ban(
        &mut self,
        target: Entity<TargetSelector>,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = std::io::Result<String>> + Send
    where
        Self: Send,
    {
        let mut args = vec!["ban".into(), target.serialize()];

        if let Some(reason) = reason {
            args.push(reason);
        }

        async move { self.run_utf8(&args).await }
    }

    /// Adds IP address to banlist.
    ///
    /// Specifies the IP address to be added to the blacklist.
    /// Can also be a name of an online player, which represents the IP of that player.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occur
    /// or if the returned bytes are not valid UTF-8.
    fn ban_ip(
        &mut self,
        target: ban_ip::Target,
        reason: Option<Cow<'_, str>>,
    ) -> impl Future<Output = Result<(), ban_ip::Error>> + Send
    where
        Self: Send,
    {
        let mut args = vec!["ban_ip".into(), target.serialize()];

        if let Some(reason) = reason {
            args.push(reason);
        }

        async move { ban_ip::parse_response(&self.run_utf8(&args).await?) }
    }

    /// Return the entries from the ban list.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occur or if the returned data is invalid.
    fn banlist(
        &mut self,
        entry_type: Option<banlist::EntryType>,
    ) -> impl Future<Output = Result<Vec<banlist::Entry>, banlist::Error>> + Send
    where
        Self: Send,
    {
        let mut args = vec!["banlist".into()];

        if let Some(entry_type) = entry_type {
            args.push(entry_type.serialize());
        }

        async move { banlist::parse_entries(&self.run_utf8(&args).await?) }
    }
}

impl<T> JavaEdition for T where T: Minecraft {}
