use crate::minecraft::entity::Entity;
use crate::minecraft::serialize::Serialize;
use crate::minecraft::util::parse_response;
use crate::Minecraft;

pub use grant::Grant;

mod grant;

/// A proxy object to allow executing advancement-related commands
/// pertaining to the selected target.
#[derive(Debug)]
pub struct Proxy<'client, T>
where
    T: Minecraft,
{
    client: &'client mut T,
    target: Entity,
}

impl<'client, T> Proxy<'client, T>
where
    T: Minecraft,
{
    pub(crate) fn new(client: &'client mut T, target: Entity) -> Self {
        Proxy { client, target }
    }

    /// Grant some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if granting the advancement fails.
    pub async fn grant(&mut self, grant: Grant) -> std::io::Result<String>
    where
        T: Send,
    {
        self.client
            .run_utf8(&["grant".into(), self.target.serialize(), grant.serialize()])
            .await
            .and_then(parse_response)
    }

    /// Revoke some advancement.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if revoking the advancement fails.
    pub async fn revoke(&mut self, grant: Grant) -> std::io::Result<String>
    where
        T: Send,
    {
        self.client
            .run_utf8(&["revoke".into(), self.target.serialize(), grant.serialize()])
            .await
            .and_then(parse_response)
    }
}
