//! `RCON` client extensions for the `Minecraft: Bedrock Edition`.

use std::future::Future;

use crate::minecraft::{parse_response, Error};
use crate::Minecraft;

/// Extension trait for `Source RCON` clients for the `Minecraft: Bedrock Edition`.
pub trait BedrockEdition: Minecraft {
    /// Locks and unlocks eternal daytime.
    fn day_lock(&mut self, lock: bool) -> impl Future<Output = Result<String, Error>> + Send;

    /// Locks and unlocks eternal daytime.
    fn always_day(&mut self, lock: bool) -> impl Future<Output = Result<String, Error>> + Send;
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
}
