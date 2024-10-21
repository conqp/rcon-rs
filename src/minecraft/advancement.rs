use crate::minecraft::entity::Entity;
use crate::minecraft::serialize::Serialize;
use crate::minecraft::util::parse_response;
use crate::Minecraft;

use grant::Grant;

mod grant;

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

    async fn grant(&mut self, grant: Grant) -> std::io::Result<String>
    where
        T: Send,
    {
        self.client
            .run_utf8(&["grant".into(), self.target.serialize(), grant.serialize()])
            .await
            .and_then(parse_response)
    }

    async fn revoke(&mut self, grant: Grant) -> std::io::Result<String>
    where
        T: Send,
    {
        self.client
            .run_utf8(&["revoke".into(), self.target.serialize(), grant.serialize()])
            .await
            .and_then(parse_response)
    }
}
