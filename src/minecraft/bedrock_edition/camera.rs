use std::borrow::Cow;
use std::future::Future;

pub use color::Color;
pub use set::{Set, Target};
pub use time::Time;

use crate::minecraft::proxy::Proxy;
use crate::minecraft::{Error, Serialize};
use crate::RCon;

mod color;
mod set;
mod time;

/// Camera-related operations.
pub trait Camera<'client> {
    /// Return a proxy with available set options.
    fn set(self, preset: Cow<'client, str>) -> impl Set;

    /// Clear the camera view.
    fn clear(&mut self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Fade camera view.
    fn fade(
        &mut self,
        color: Color,
        time: Option<Time>,
    ) -> impl Future<Output = Result<String, Error>> + Send;
}

impl<'client, T> Camera<'client> for Proxy<'client, T>
where
    T: RCon + Send,
{
    fn set(self, preset: Cow<'client, str>) -> impl Set {
        self.delegate(&[Cow::Borrowed("set"), preset])
    }

    async fn clear(&mut self) -> Result<String, Error> {
        self.run_utf8(&["clear".into()]).await
    }

    async fn fade(&mut self, color: Color, time: Option<Time>) -> Result<String, Error> {
        let mut args = vec![Cow::Borrowed("fade")];

        if let Some(time) = time {
            args.push(time.serialize());
        }

        args.push(color.serialize());
        self.run_utf8(&args).await
    }
}
