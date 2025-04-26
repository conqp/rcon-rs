//! Creates, modifies and lists bossbars.

use crate::minecraft::{JavaEdition, ResourceLocation};
use crate::Error;

pub use get_target::GetTarget;

mod get_target;

/// A proxy object to handle bossbar-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
}

impl<'client, T> Proxy<'client, T>
where
    T: JavaEdition + Send,
{
    #[must_use]
    pub(crate) const fn new(client: &'client mut T) -> Self {
        Self { client }
    }

    /// Create a new bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn add(&mut self, id: ResourceLocation, name: String) -> Result<String, Error> {
        self.client
            .run_utf8(format!("bossbar add {id} {name}"))
            .await
    }

    /// Return the requested setting as a `result` of the command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn get(&mut self, id: ResourceLocation, target: GetTarget) -> Result<String, Error> {
        self.client
            .run_utf8(format!("bossbar get {id} {target}"))
            .await
    }

    /// Display a list of existing bossbars.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn list(&mut self) -> Result<String, Error> {
        // TODO: Parse output into bossbar list object.
        self.client.run_utf8("bossbar list").await
    }

    /// Remove an existing bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn remove(&mut self, id: ResourceLocation) -> Result<String, Error> {
        self.client.run_utf8(format!("bossbar remove {id}")).await
    }
}
