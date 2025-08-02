use std::borrow::Cow;

use crate::minecraft::Error;
use crate::RCon;

/// A proxy object to handle sub-commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    args: Vec<Cow<'client, str>>,
}

impl<'client, T> Proxy<'client, T> {
    /// Create a new proxy.
    #[must_use]
    pub(crate) const fn new(client: &'client mut T, args: Vec<Cow<'client, str>>) -> Self {
        Self { client, args }
    }

    /// Delegate to a sub-proxy by extending the args and returning `self`.
    ///
    /// The caller should return an opaque type implementing a certain impl trait with the desired functionality.
    pub(crate) fn delegate(mut self, args: &[Cow<'client, str>]) -> Self {
        self.args.extend_from_slice(args);
        self
    }
}

impl<T> Proxy<'_, T>
where
    T: RCon + Send,
{
    /// Run a command on the underlying `RCON` client.
    pub(crate) async fn run_utf8(&mut self, args: &[Cow<'_, str>]) -> Result<String, Error> {
        self.client
            .run_utf8([&self.args, args].concat().join(" "))
            .await
            .map_err(Into::into)
    }
}
