//! `RCON` client extensions for the `Minecraft: Bedrock Edition`.

use std::future::Future;

use crate::minecraft::util::parse_response;
use crate::Minecraft;

/// Extension trait for `Source RCON` clients for the `Minecraft: Bedrock Edition`.
pub trait BedrockEdition: Minecraft {
    /// Locks and unlocks eternal daytime.
    fn day_lock(&mut self, lock: bool) -> impl Future<Output = std::io::Result<String>> + Send;

    /// Locks and unlocks eternal daytime.
    fn always_day(&mut self, lock: bool) -> impl Future<Output = std::io::Result<String>> + Send;
}

impl<T> BedrockEdition for T
where
    T: Minecraft + Send,
{
    async fn day_lock(&mut self, lock: bool) -> std::io::Result<String> {
        self.run_utf8(&["daylock".into(), lock.to_string().into()])
            .await
            .and_then(parse_response)
    }

    async fn always_day(&mut self, lock: bool) -> std::io::Result<String> {
        self.run_utf8(&["alwaysday".into(), lock.to_string().into()])
            .await
            .and_then(parse_response)
    }
}
