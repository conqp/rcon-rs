use crate::minecraft::target_selector::TargetSelector;
use crate::Minecraft;

use ability::Ability;

mod ability;

pub struct AbilitiesProxy<'client, T>
where
    T: Minecraft,
{
    client: &'client mut T,
    target: TargetSelector,
}

impl<'client, T> AbilitiesProxy<'client, T>
where
    T: Minecraft + Send,
{
    pub(crate) fn new(client: &'client mut T, target: TargetSelector) -> Self {
        Self { client, target }
    }

    pub async fn iter(&mut self) -> std::io::Result<Vec<Ability>> {
        self.client
            .run_utf8(&["ability".into()])
            .await
            .map(|text| todo!("Parse"))
    }

    /// Returns whether the target_selector's ability is set.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn get(&mut self, ability: Ability) -> std::io::Result<bool> {
        todo!()
    }

    /// Enables the given ability on the target_selector.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn enable(&mut self, ability: Ability) -> std::io::Result<bool> {
        todo!()
    }

    /// Disables the given ability on the target_selector.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if fetching the available commands fails.
    pub async fn disable(&mut self, ability: Ability) -> std::io::Result<bool> {
        todo!()
    }
}
