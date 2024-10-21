use crate::minecraft::entity::Entity;
use crate::Minecraft;

use crate::minecraft::serialize::Serialize;
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
    }

    async fn revoke(&mut self, grant: Grant) -> std::io::Result<()>
    where
        T: Send,
    {
        todo!()
    }
}
