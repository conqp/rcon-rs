//! Gives, removes, or checks player advancements.

use std::borrow::Cow;

pub use grant::Grant;

use crate::minecraft::{parse_response, Serialize};
use crate::{minecraft, RCon};

mod grant;

/// A proxy object to allow executing advancement-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    args: Vec<Cow<'client, str>>,
}

impl<'client, T> Proxy<'client, T> {
    pub(crate) const fn new(client: &'client mut T, args: Vec<Cow<'client, str>>) -> Self {
        Proxy { client, args }
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
    pub async fn grant(mut self, grant: Grant) -> Result<String, minecraft::Error> {
        self.args.extend(["grant".into(), grant.serialize().into()]);
        parse_response(self.client.run_utf8(self.args.join(" ")).await?)
    }

    /// Revoke some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if revoking the advancement fails.
    pub async fn revoke(mut self, grant: Grant) -> Result<String, minecraft::Error> {
        self.args
            .extend(["revoke".into(), grant.serialize().into()]);
        parse_response(self.client.run_utf8(self.args.join(" ")).await?)
    }
}
