//! `Source RCON` client extensions for Minecraft servers.

use std::future::Future;

#[cfg(feature = "minecraft-bedrock-edition")]
pub use self::bedrock_edition::BedrockEdition;
#[cfg(feature = "minecraft-education-edition")]
pub use self::education_edition::EducationEdition;
pub use self::entity::Entity;
pub use self::error::Error;
pub use self::game_mode::GameMode;
#[cfg(feature = "minecraft-java-edition")]
pub use self::java_edition::JavaEdition;
pub use self::negate::Negate;
pub use self::range::Range;
pub use self::resource_location::ResourceLocation;
pub use self::serialize::Serialize;
pub use self::unsigned_float::UnsignedFloat;
use self::util::parse_response;
use crate::source::Source;

#[cfg(feature = "minecraft-bedrock-edition")]
pub mod bedrock_edition;
#[cfg(feature = "minecraft-education-edition")]
pub mod education_edition;
mod entity;
mod error;
mod game_mode;
#[cfg(feature = "minecraft-java-edition")]
pub mod java_edition;
mod negate;
mod proxy;
mod range;
mod resource_location;
mod serialize;
mod unsigned_float;
mod util;

/// Extension trait for `Source RCON` clients for generic Minecraft servers.
pub trait Minecraft: Source {
    /// Print information about available commands on the server.
    ///
    /// If the optional parameter `command` is provided, list help about that specific command.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    fn help<T>(
        &mut self,
        command: Option<T>,
    ) -> impl Future<Output = Result<String, crate::Error>> + Send
    where
        T: AsRef<str> + Send;
}

impl<T> Minecraft for T
where
    T: Source + Send,
{
    async fn help<S>(&mut self, command: Option<S>) -> Result<String, crate::Error>
    where
        S: AsRef<str> + Send,
    {
        let mut args = vec!["help"];

        if let Some(command) = &command {
            args.push(command.as_ref());
        }

        self.run_utf8(args.join(" ")).await
    }
}
