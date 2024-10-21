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
    pub async fn get(self, scale: Option<f64>) -> std::io::Result<String> {
        let mut args = vec![
            "attribute".into(),
            self.target.serialize(),
            self.attribute.serialize(),
            "get".into(),
        ];

        if let Some(scale) = scale {
            args.push(scale.serialize().to_string().into());
        }

        self.client.run_utf8(args.as_slice()).await
    }

    /// Returns the target's base attribute value.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn base(self, scale: Option<f64>) -> std::io::Result<String> {
        let mut args = vec![
            "attribute".into(),
            self.target.serialize(),
            self.attribute.serialize(),
            "base".into(),
            "get".into(),
        ];

        if let Some(scale) = scale {
            args.push(scale.serialize());
        }

        self.client.run_utf8(args.as_slice()).await
    }
}
