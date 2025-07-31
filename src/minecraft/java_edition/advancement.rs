//! Gives, removes, or checks player advancements.

pub use grant::Grant;

use super::TargetSelector;
use crate::minecraft::{parse_response, Entity, Serialize};
use crate::{minecraft, RCon};

mod grant;

/// A proxy object to allow executing advancement-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    target: Entity<TargetSelector>,
}

impl<'client, T> Proxy<'client, T> {
    pub(crate) const fn new(client: &'client mut T, target: Entity<TargetSelector>) -> Self {
        Proxy { client, target }
    }
}

impl<T> Proxy<'_, T>
where
    T: RCon + Send,
{
    /// Grant some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if granting the advancement fails.
    pub async fn grant(self, grant: Grant) -> Result<String, minecraft::Error> {
        parse_response(
            self.client
                .run_utf8(format!(
                    "grant {} {}",
                    self.target.serialize(),
                    grant.serialize()
                ))
                .await?,
        )
    }

    /// Revoke some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if revoking the advancement fails.
    pub async fn revoke(self, grant: Grant) -> Result<String, minecraft::Error> {
        parse_response(
            self.client
                .run_utf8(format!(
                    "revoke {} {}",
                    self.target.serialize(),
                    grant.serialize()
                ))
                .await?,
        )
    }
}
