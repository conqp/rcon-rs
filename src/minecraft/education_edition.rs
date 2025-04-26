//! TODO: implement education edition extensions.

use crate::minecraft::BedrockEdition;
use target_selector::TargetSelector;

mod ability;
mod target_selector;

/// Extension trait for `Source RCON` clients for the `Minecraft: Education Edition`.
pub trait EducationEdition: BedrockEdition {
    /// Manage the target's ability.
    ///
    /// # Returns
    ///
    /// Returns an [`ability::Proxy`] which can be used to execute
    /// ability-related commands pertaining to the `target`.
    fn ability(&mut self, target: TargetSelector) -> ability::Proxy<'_, Self>
    where
        Self: Sized + Send,
    {
        ability::Proxy::new(self, target)
    }
}

impl<T> EducationEdition for T where T: BedrockEdition {}
