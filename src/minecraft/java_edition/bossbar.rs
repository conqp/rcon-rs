//! Creates, modifies and lists bossbars.

use crate::minecraft::{JavaEdition, ResourceLocation};
use crate::Error;

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
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn add(&mut self, id: ResourceLocation, name: String) -> Result<String, Error> {
        self.client
            .run_utf8(format!("bossbar add {id} {name}"))
            .await
    }
}
