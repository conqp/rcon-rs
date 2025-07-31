use std::borrow::Cow;

pub use color::Color;
pub use time::Time;

use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{Entity, Error, Serialize};
use crate::RCon;

mod color;
mod set;
mod time;

/// Camera actions proxy.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    target: Entity<TargetSelector>,
}

impl<'client, T> Proxy<'client, T> {
    pub(crate) const fn new(client: &'client mut T, target: Entity<TargetSelector>) -> Self {
        Self { client, target }
    }

    /// Return a proxy with available set options.
    pub fn set<'preset>(self, preset: Cow<'preset, str>) -> set::Proxy<'client, 'preset, T> {
        set::Proxy::new(self.client, preset)
    }
}

impl<T> Proxy<'_, T>
where
    T: RCon + Send,
{
    /// Clear the camera view.
    pub async fn clear(self) -> Result<String, Error> {
        self.client
            .run_utf8(format!("{} clear", self.target.serialize()))
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
