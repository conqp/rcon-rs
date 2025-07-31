//! `RCON` client extensions for the `Minecraft: Bedrock Edition`.

mod camera;

use std::future::Future;

use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{parse_response, Entity, Error};
use crate::Minecraft;

/// Extension trait for `Source RCON` clients for the `Minecraft: Bedrock Edition`.
pub trait BedrockEdition: Minecraft {
    /// Locks and unlocks eternal daytime.
    fn day_lock(&mut self, lock: bool) -> impl Future<Output = Result<String, Error>> + Send;

    /// Locks and unlocks eternal daytime.
    fn always_day(&mut self, lock: bool) -> impl Future<Output = Result<String, Error>> + Send;

    /// Modify the player's camera view.
    fn camera(&mut self, target: Entity<TargetSelector>) -> camera::Proxy<'_, Self>
    where
        Self: Sized + Send;
}

impl<T> BedrockEdition for T
where
    T: Minecraft + Send,
{
    async fn day_lock(&mut self, lock: bool) -> Result<String, Error> {
        self.run_utf8(format!("daylock {lock}"))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }

    async fn always_day(&mut self, lock: bool) -> Result<String, Error> {
        self.run_utf8(format!("alwaysday {lock}"))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }

    fn camera(&mut self, target: Entity<TargetSelector>) -> camera::Proxy<'_, Self> {
        camera::Proxy::new(self, target)
    }
}
