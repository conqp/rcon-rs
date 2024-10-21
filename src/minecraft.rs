//! `Source RCON` client extensions for Minecraft servers.

use std::borrow::Cow;
use std::future::Future;

use crate::source::Source;
use crate::RCon;

use abilities::AbilitiesProxy;
use target_selector::TargetSelector;

#[cfg(feature = "education")]
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

    /// Returns the target_selector's abilities.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    #[cfg(feature = "education")]
    fn abilities(&mut self, target: TargetSelector) -> AbilitiesProxy<'_, Self>
    where
        Self: Sized + Send,
    {
        AbilitiesProxy::new(self, target)
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
