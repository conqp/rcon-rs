use std::borrow::Cow;

pub use color::Color;
pub use time::Time;

use crate::minecraft::{Error, Serialize};
use crate::RCon;

mod color;
mod set;
mod time;

/// Camera actions proxy.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    args: Vec<Cow<'client, str>>,
}

impl<'client, T> Proxy<'client, T> {
    pub(crate) fn new(client: &'client mut T, args: Vec<Cow<'client, str>>) -> Self {
        Self { client, args }
    }

    /// Return a proxy with available set options.
    pub fn set(mut self, preset: Cow<'client, str>) -> set::Proxy<'client, T> {
        self.args.extend([Cow::Borrowed("set"), preset]);
        set::Proxy::new(self.client, self.args)
    }
}

impl<T> Proxy<'_, T>
where
    T: RCon + Send,
{
    /// Clear the camera view.
    pub async fn clear(mut self) -> Result<String, Error> {
        self.args.push(Cow::Borrowed("clear"));
        self.client
            .run_utf8(self.args.join(" "))
            .await
            .map_err(Into::into)
    }

    /// Fade camera view.
    pub async fn fade(self, color: Color, time: Option<Time>) -> Result<String, Error> {
        let mut args = vec![Cow::Borrowed("fade")];

        if let Some(time) = time {
            args.push(time.serialize());
        }

        args.push(color.serialize());
        self.client
            .run_utf8(args.join(" "))
            .await
            .map_err(Into::into)
    }
}
