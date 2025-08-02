use std::borrow::Cow;

use ability::Ability;

use crate::minecraft::{parse_response, Error, Serialize};
use crate::RCon;

#[allow(clippy::module_inception)]
mod ability;

/// A proxy object to handle ability-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    args: Vec<Cow<'client, str>>,
}

impl<'client, T> Proxy<'client, T> {
    pub(crate) const fn new(client: &'client mut T, args: Vec<Cow<'client, str>>) -> Self {
        Self { client, args }
    }
}

impl<T> Proxy<'_, T>
where
    T: RCon + Send,
{
    /// List the target's ability.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn list(self) -> Result<Vec<String>, Error> {
        self.client
            .run_utf8(self.args.join(" "))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
            // TODO: How to parse this?
            .map(|text| text.lines().map(ToString::to_string).collect())
    }

    /// Returns whether the target's ability is set.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn get(mut self, ability: Ability) -> Result<String, Error> {
        self.args.push(ability.serialize());
        self.client
            .run_utf8(self.args.join(" "))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }

    /// Enables the given ability on the target.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn enable(mut self, ability: Ability) -> Result<String, Error> {
        self.args.extend([ability.serialize(), true.serialize()]);
        self.client
            .run_utf8(self.args.join(" "))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }

    /// Disables the given ability on the target.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn disable(mut self, ability: Ability) -> Result<String, Error> {
        self.args.extend([ability.serialize(), false.serialize()]);
        self.client
            .run_utf8(self.args.join(" "))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }
}
