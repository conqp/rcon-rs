//! `RCON` extensions interface for Minecraft: Java Edition servers.

use crate::minecraft::Entity;
use crate::Minecraft;

pub use advancement::Grant;
pub use target_selector::{Argument, Sort, TargetSelector};

pub mod advancement;
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
}
