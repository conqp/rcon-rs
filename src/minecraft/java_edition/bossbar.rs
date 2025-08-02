//! Creates, modifies and lists bossbars.

use std::borrow::Cow;

pub use get_target::GetTarget;
pub use set_target::{Color, SetTarget, Style};

use crate::minecraft::{ResourceLocation, Serialize};
use crate::{Error, RCon};

mod get_target;
mod set_target;

/// A proxy object to handle bossbar-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    args: Vec<Cow<'client, str>>,
}

impl<'client, T> Proxy<'client, T> {
    #[must_use]
    pub(crate) const fn new(client: &'client mut T, args: Vec<Cow<'client, str>>) -> Self {
        Self { client, args }
    }
}

impl<T> Proxy<'_, T>
where
    T: RCon + Send,
{
    /// Create a new bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn add(mut self, id: ResourceLocation, name: String) -> Result<String, Error> {
        self.args
            .extend(["add".into(), id.serialize(), name.serialize()]);
        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Return the requested setting as a `result` of the command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn get(mut self, id: ResourceLocation, target: GetTarget) -> Result<String, Error> {
        self.args
            .extend(["get".into(), id.serialize(), target.serialize()]);
        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Display a list of existing bossbars.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn list(mut self) -> Result<String, Error> {
        // TODO: Parse output into bossbar list object.
        self.args.push("list".into());
        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Remove an existing bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn remove(mut self, id: ResourceLocation) -> Result<String, Error> {
        self.args.extend(["remove".into(), id.serialize()]);
        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Set the respective value of the bossbar.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any errors occurred.
    pub async fn set(mut self, id: ResourceLocation, target: SetTarget) -> Result<String, Error> {
        self.args
            .extend(["set".into(), id.serialize(), target.serialize()]);
        self.client.run_utf8(self.args.join(" ")).await
    }
}
