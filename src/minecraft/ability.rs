use crate::minecraft::serialize::Serialize;
use crate::minecraft::target_selector::TargetSelector;
use crate::minecraft::util::parse_response;
use crate::Minecraft;

use ability::Ability;

mod ability;

pub struct Proxy<'client, T>
where
    T: Minecraft,
{
    client: &'client mut T,
    target: TargetSelector,
}

impl<'client, T> Proxy<'client, T>
where
    T: Minecraft + Send,
{
    pub(crate) fn new(client: &'client mut T, target: TargetSelector) -> Self {
        Self { client, target }
    }

    /// List the target's ability.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn list(&mut self) -> std::io::Result<Vec<String>> {
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
    pub async fn get(&mut self, _ability: Ability) -> std::io::Result<String> {
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
    pub async fn enable(&mut self, ability: Ability) -> std::io::Result<String> {
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
    pub async fn disable(&mut self, ability: Ability) -> std::io::Result<String> {
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
