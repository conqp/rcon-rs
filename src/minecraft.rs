//! `Source RCON` client extensions for Minecraft servers.

use std::borrow::Cow;
use std::future::Future;

use crate::source::Source;
use crate::RCon;

pub use advancement::Grant;
pub use entity::Entity;
pub use game_mode::GameMode;
pub use negate::Negate;
pub use range::Range;
pub use resource_location::ResourceLocation;
pub use target_selector::TargetSelector;
pub use unsigned_float::UnsignedFloat;

#[cfg(feature = "minecraft-education-edition")]
mod abilities;
mod advancement;
mod entity;
mod game_mode;
mod negate;
mod range;
mod resource_location;
mod serialize;
mod target_selector;
mod unsigned_float;
mod util;

/// Extension trait for `Source RCON` clients for Minecraft servers.
pub trait Minecraft: RCon + Source {
    /// Print information about available commands on the server.
    ///
    /// If the optional parameter `command` is provided, list help about that specific command.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    fn help(
        &mut self,
        command: Option<Cow<'_, str>>,
    ) -> impl Future<Output = std::io::Result<String>> + Send;

    /// Manage the target's abilities.
    ///
    /// # Returns
    ///
    /// Returns an [`abilities::Proxy`] which can be used to execute
    /// ability-related commands pertaining to the `target`.
    #[cfg(feature = "minecraft-education-edition")]
    fn ability(&mut self, target: TargetSelector) -> abilities::Proxy<'_, Self>
    where
        Self: Sized + Send,
    {
        abilities::Proxy::new(self, target)
    }

    /// Manage the target's abilities.
    ///
    /// # Returns
    ///
    /// Returns an [`advancement::Proxy`] which can be used to execute
    /// advancement-related commands pertaining to the `target`.
    fn advancement(&mut self, target: Entity) -> advancement::Proxy<'_, Self>
    where
        Self: Sized + Send,
    {
        advancement::Proxy::new(self, target)
    }
}

impl<T> Minecraft for T
where
    T: RCon + Source + Send,
{
    async fn help(&mut self, command: Option<Cow<'_, str>>) -> std::io::Result<String> {
        if let Some(command) = command {
            self.run_utf8(&["help".into(), command]).await
        } else {
            self.run_utf8(&["help".into()]).await
        }
    }
}
