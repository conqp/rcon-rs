use ability::Ability;

use super::target_selector::TargetSelector;
use crate::minecraft::{parse_response, Error, Serialize};
use crate::RCon;

#[allow(clippy::module_inception)]
mod ability;

/// A proxy object to handle ability-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    target: TargetSelector,
}

impl<'client, T> Proxy<'client, T> {
    pub(crate) const fn new(client: &'client mut T, target: TargetSelector) -> Self {
        Self { client, target }
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
            .run_utf8(format!("ability {}", self.target.serialize()))
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
    pub async fn get(self, ability: Ability) -> Result<String, Error> {
        self.client
            .run_utf8(format!(
                "ability {} {}",
                self.target.serialize(),
                ability.serialize()
            ))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }

    /// Enables the given ability on the target.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn enable(self, ability: Ability) -> Result<String, Error> {
        self.client
            .run_utf8(format!(
                "ability {} {} {}",
                self.target.serialize(),
                ability.serialize(),
                true,
            ))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }

    /// Disables the given ability on the target.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn disable(self, ability: Ability) -> Result<String, Error> {
        self.client
            .run_utf8(format!(
                "ability {} {} {}",
                self.target.serialize(),
                ability.serialize(),
                false,
            ))
            .await
            .map_err(Into::into)
            .and_then(parse_response)
    }
}
