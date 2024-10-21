use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{Entity, JavaEdition, ResourceLocation, Serialize};

/// A proxy object to handle attribute-related commands.
pub struct Proxy<'client, T>
where
    T: JavaEdition,
{
    client: &'client mut T,
    target: Entity<TargetSelector>,
    attribute: ResourceLocation,
}

impl<'client, T> Proxy<'client, T>
where
    T: JavaEdition + Send,
{
    pub(crate) fn new(
        client: &'client mut T,
        target: Entity<TargetSelector>,
        attribute: ResourceLocation,
    ) -> Self {
        Self {
            client,
            target,
            attribute,
        }
    }

    /// Returns the target's attribute value.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn get(&mut self, scale: Option<f64>) -> std::io::Result<String> {
        if let Some(scale) = scale {
            self.client
                .run_utf8(&[
                    "attribute".into(),
                    self.target.serialize(),
                    self.attribute.serialize(),
                    "get".into(),
                    scale.serialize(),
                ])
                .await
        } else {
            self.client
                .run_utf8(&[
                    "attribute".into(),
                    self.target.serialize(),
                    self.attribute.serialize(),
                    "get".into(),
                ])
                .await
        }
    }

    /// Returns the target's base attribute value.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn base(&mut self, scale: Option<f64>) -> std::io::Result<String> {
        if let Some(scale) = scale {
            self.client
                .run_utf8(&[
                    "attribute".into(),
                    self.target.serialize(),
                    self.attribute.serialize(),
                    "base".into(),
                    "get".into(),
                    scale.serialize(),
                ])
                .await
        } else {
            self.client
                .run_utf8(&[
                    "attribute".into(),
                    self.target.serialize(),
                    self.attribute.serialize(),
                    "base".into(),
                    "get".into(),
                ])
                .await
        }
    }
}
