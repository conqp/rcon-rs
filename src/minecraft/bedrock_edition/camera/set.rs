use crate::minecraft::{Error, Serialize};
use crate::RCon;
use std::borrow::Cow;
pub use target::Target;

mod target;

/// Camera actions proxy.
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
    /// Set the default value.
    ///
    /// TODO: investigate what this actually is. The wiki is quite sparse on this.
    ///
    /// See: <https://minecraft.fandom.com/wiki/Commands/camera>
    pub async fn default(mut self, default: Option<Cow<'_, str>>) -> Result<String, Error> {
        self.args.push(Cow::Borrowed("default"));

        if let Some(default) = default {
            self.args.push(default);
        }

        self.client
            .run_utf8(self.args.join(" "))
            .await
            .map_err(Into::into)
    }

    /// Set camera facing.
    pub async fn facing(mut self, target: Target) -> Result<String, Error> {
        self.args
            .extend([Cow::Borrowed("facing"), target.serialize()]);
        self.client
            .run_utf8(self.args.join(" "))
            .await
            .map_err(Into::into)
    }

    // TODO: Implement further sub-commands.
}
