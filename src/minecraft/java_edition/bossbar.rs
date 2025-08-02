//! Creates, modifies and lists bossbars.

use std::future::Future;

pub use get_target::GetTarget;
pub use set_target::{Color, SetTarget, Style};

use crate::minecraft::proxy::Proxy;
use crate::minecraft::{Error, ResourceLocation, Serialize};
use crate::RCon;

mod get_target;
mod set_target;

/// Bossbar-related operations.
pub trait Bossbar {
    /// Create a new bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    fn add(
        &mut self,
        id: ResourceLocation,
        name: String,
    ) -> impl Future<Output = Result<String, Error>> + Send;

    /// Return the requested setting as a `result` of the command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    fn get(
        &mut self,
        id: ResourceLocation,
        target: GetTarget,
    ) -> impl Future<Output = Result<String, Error>> + Send;

    /// Display a list of existing bossbars.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    fn list(&mut self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Remove an existing bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    fn remove(
        &mut self,
        id: ResourceLocation,
    ) -> impl Future<Output = Result<String, Error>> + Send;

    /// Set the respective value of the bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    fn set(
        &mut self,
        id: ResourceLocation,
        target: SetTarget,
    ) -> impl Future<Output = Result<String, Error>> + Send;
}

impl<T> Bossbar for Proxy<'_, T>
where
    T: RCon + Send,
{
    async fn add(&mut self, id: ResourceLocation, name: String) -> Result<String, Error> {
        self.run_utf8(&["add".into(), id.serialize(), name.serialize()])
            .await
    }

    async fn get(&mut self, id: ResourceLocation, target: GetTarget) -> Result<String, Error> {
        self.run_utf8(&["get".into(), id.serialize(), target.serialize()])
            .await
    }

    async fn list(&mut self) -> Result<String, Error> {
        // TODO: Parse output into bossbar list object.
        self.run_utf8(&["list".into()]).await
    }

    async fn remove(&mut self, id: ResourceLocation) -> Result<String, Error> {
        self.run_utf8(&["remove".into(), id.serialize()]).await
    }

    async fn set(&mut self, id: ResourceLocation, target: SetTarget) -> Result<String, Error> {
        self.run_utf8(&["set".into(), id.serialize(), target.serialize()])
            .await
    }
}
