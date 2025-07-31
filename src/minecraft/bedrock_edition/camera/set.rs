use std::borrow::Cow;

use crate::minecraft::Error;
use crate::RCon;

/// Camera actions proxy.
#[derive(Debug)]
pub struct Proxy<'client, 'preset, T> {
    client: &'client mut T,
    preset: Cow<'preset, str>,
}

impl<'client, 'preset, T> Proxy<'client, 'preset, T> {
    pub(crate) const fn new(client: &'client mut T, preset: Cow<'preset, str>) -> Self {
        Self { client, preset }
    }
}

impl<T> Proxy<'_, '_, T>
where
    T: RCon + Send,
{
    /// Set the default value.
    ///
    /// TODO: investigate what this actually is. The wiki is quite sparse on this.
    ///
    /// See: <https://minecraft.fandom.com/wiki/Commands/camera>
    pub async fn default(self, default: Option<Cow<'_, str>>) -> Result<String, Error> {
        let mut args = vec![Cow::Borrowed("set"), self.preset];

        if let Some(default) = default {
            args.push(default);
        }

        self.client
            .run_utf8(args.join(" "))
            .await
            .map_err(Into::into)
    }

    // TODO: Implement further sub-commands.
}
