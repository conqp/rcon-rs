use crate::minecraft::parse_response;
use crate::minecraft::Serialize;

use super::target_selector::TargetSelector;
use super::EducationEdition;

use ability::Ability;

#[allow(clippy::module_inception)]
mod ability;

pub struct Proxy<'client, T>
where
    T: EducationEdition,
{
    client: &'client mut T,
    target: TargetSelector,
}

impl<'client, T> Proxy<'client, T>
where
    T: EducationEdition + Send,
{
    pub(crate) fn new(client: &'client mut T, target: TargetSelector) -> Self {
        Self { client, target }
    }

    /// List the target's ability.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn list(self) -> std::io::Result<Vec<String>> {
        self.client
            .run_utf8(&["ability".into(), self.target.serialize()])
            .await
            .and_then(parse_response)
            // TODO: How to parse this?
            .map(|text| text.lines().map(ToString::to_string).collect())
    }

    /// Returns whether the target's ability is set.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn get(self, _ability: Ability) -> std::io::Result<String> {
        self.client
            .run_utf8(&["ability".into(), self.target.serialize()])
            .await
            .and_then(parse_response)
    }

    /// Enables the given ability on the target.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn enable(self, ability: Ability) -> std::io::Result<String> {
        self.client
            .run_utf8(&[
                "ability".into(),
                self.target.serialize(),
                ability.serialize(),
                true.to_string().into(),
            ])
            .await
            .and_then(parse_response)
    }

    /// Disables the given ability on the target.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn disable(self, ability: Ability) -> std::io::Result<String> {
        self.client
            .run_utf8(&[
                "ability".into(),
                self.target.serialize(),
                ability.serialize(),
                false.to_string().into(),
            ])
            .await
            .and_then(parse_response)
    }
}
