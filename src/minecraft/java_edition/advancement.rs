//! Gives, removes, or checks player advancements.

use std::future::Future;

pub use grant::Grant;

use crate::minecraft::proxy::Proxy;
use crate::minecraft::{parse_response, Serialize};
use crate::{minecraft, RCon};

mod grant;

/// Advancement-related operations.
pub trait Advancement {
    /// Grant some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if granting the advancement fails.
    fn grant(
        &mut self,
        grant: Grant,
    ) -> impl Future<Output = Result<String, minecraft::Error>> + Send;

    /// Revoke some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if revoking the advancement fails.
    fn revoke(
        &mut self,
        grant: Grant,
    ) -> impl Future<Output = Result<String, minecraft::Error>> + Send;
}

impl<T> Advancement for Proxy<'_, T>
where
    T: RCon + Send,
{
    async fn grant(&mut self, grant: Grant) -> Result<String, minecraft::Error> {
        parse_response(self.run_utf8(&["grant".into(), grant.serialize()]).await?)
    }

    async fn revoke(&mut self, grant: Grant) -> Result<String, minecraft::Error> {
        parse_response(self.run_utf8(&["revoke".into(), grant.serialize()]).await?)
    }
}
