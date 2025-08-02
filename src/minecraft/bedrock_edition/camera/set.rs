use std::borrow::Cow;
use std::future::Future;

pub use target::Target;

use crate::minecraft::proxy::Proxy;
use crate::minecraft::util::EscapeString;
use crate::minecraft::{Error, Serialize};
use crate::RCon;

mod target;

/// Set camera settings.
pub trait Set {
    /// Set the default value.
    ///
    /// TODO: investigate what this actually is. The wiki is quite sparse on this.
    ///
    /// See: <https://minecraft.fandom.com/wiki/Commands/camera>
    fn default(
        &mut self,
        default: Option<Cow<'_, str>>,
    ) -> impl Future<Output = Result<String, Error>>;

    /// Set camera facing.
    fn facing(&mut self, target: Target) -> impl Future<Output = Result<String, Error>>;

    // TODO: Implement further sub-commands.
}

impl<T> Set for Proxy<'_, T>
where
    T: RCon + Send,
{
    async fn default(&mut self, default: Option<Cow<'_, str>>) -> Result<String, Error> {
        let mut args = vec![Cow::Borrowed("default")];

        if let Some(default) = default {
            args.push(default.quote().into());
        }

        self.run_utf8(&args).await
    }

    async fn facing(&mut self, target: Target) -> Result<String, Error> {
        self.run_utf8(&[Cow::Borrowed("facing"), target.serialize()])
            .await
    }
}
