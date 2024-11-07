//! Gives, removes, or checks player advancements.

use crate::minecraft::parse_response;
use crate::minecraft::Entity;
use crate::minecraft::Serialize;
use crate::Minecraft;

use super::TargetSelector;

pub use grant::Grant;

mod grant;

/// A proxy object to allow executing advancement-related commands
/// pertaining to the selected target.
#[derive(Debug)]
pub struct Proxy<'client, T>
where
    T: Minecraft,
{
    client: &'client mut T,
    target: Entity<TargetSelector>,
}

impl<'client, T> Proxy<'client, T>
where
    T: Minecraft + Send,
{
    pub(crate) fn new(client: &'client mut T, target: Entity<TargetSelector>) -> Self {
        Proxy { client, target }
    }

    /// Grant some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if granting the advancement fails.
    pub async fn grant(self, grant: Grant) -> std::io::Result<String> {
        self.client
            .run_utf8(&["grant".into(), self.target.serialize(), grant.serialize()])
            .await
            .and_then(parse_response)
    }

    /// Revoke some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if revoking the advancement fails.
    pub async fn revoke(self, grant: Grant) -> std::io::Result<String> {
        self.client
            .run_utf8(&["revoke".into(), self.target.serialize(), grant.serialize()])
            .await
            .and_then(parse_response)
    }
}
