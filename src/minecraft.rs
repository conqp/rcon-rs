//! `Source RCON` client extensions for Minecraft servers.

use std::future::Future;

#[cfg(feature = "minecraft-bedrock-edition")]
pub use bedrock_edition::BedrockEdition;
#[cfg(feature = "minecraft-education-edition")]
pub use education_edition::EducationEdition;
pub use entity::Entity;
pub use error::Error;
pub use game_mode::GameMode;
#[cfg(feature = "minecraft-java-edition")]
pub use java_edition::JavaEdition;
pub use negate::Negate;
pub use range::Range;
pub use resource_location::ResourceLocation;
pub use serialize::Serialize;
pub use unsigned_float::UnsignedFloat;
use util::parse_response;

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
