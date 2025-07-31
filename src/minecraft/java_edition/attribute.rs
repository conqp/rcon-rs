//! Queries, adds, removes or sets an entity attribute.

use std::borrow::Cow;

pub use modifier::Modifier;
use uuid::Uuid;

use crate::minecraft::java_edition::TargetSelector;
use crate::minecraft::{Entity, JavaEdition, ResourceLocation, Serialize};
use crate::Error;

mod modifier;

/// A proxy object to handle attribute-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    target: Entity<TargetSelector>,
    attribute: ResourceLocation,
}

impl<'client, T> Proxy<'client, T>
where
    T: JavaEdition + Send,
{
    pub(crate) const fn new(
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

    /// Returns the total value of the specified attribute.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn get(self, scale: Option<f64>) -> Result<String, Error> {
        let mut args = vec![
            Cow::Borrowed("attribute"),
            self.target.serialize(),
            self.attribute.serialize(),
            Cow::Borrowed("get"),
        ];

        if let Some(scale) = scale {
            args.push(Cow::Owned(scale.serialize().to_string()));
        }

        self.client.run_utf8(args.join(" ")).await
    }

    /// Returns the base value of the specified attribute.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn base(self, scale: Option<f64>) -> Result<String, Error> {
        let mut args = vec![
            Cow::Borrowed("attribute"),
            self.target.serialize(),
            self.attribute.serialize(),
            Cow::Borrowed("base"),
            Cow::Borrowed("get"),
        ];

        if let Some(scale) = scale {
            args.push(scale.serialize());
        }

        self.client.run_utf8(args.join(" ")).await
    }

    /// Overwrites the base value of the specified attribute with the given value.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn set_base(self, value: f64) -> Result<String, Error> {
        self.client
            .run_utf8(format!(
                "attribute {} {} base set {}",
                self.target.serialize(),
                self.attribute.serialize(),
                value.serialize(),
            ))
            .await
    }

    /// Adds an attribute modifier with the specified properties
    /// if no modifier with the same UUID already existed.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn add_modifier<U>(
        self,
        uuid: Uuid,
        name: U,
        value: f64,
        modifier: Modifier,
    ) -> Result<String, Error>
    where
        U: Into<Cow<'static, str>>,
    {
        self.client
            .run_utf8(format!(
                "attribute {} {} modifier add {} {} {} {}",
                self.target.serialize(),
                self.attribute.serialize(),
                uuid.serialize(),
                name.into(),
                value.serialize(),
                modifier.serialize(),
            ))
            .await
    }

    /// Removes the attribute modifier with the specified UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn remove_modifier(self, uuid: Uuid) -> Result<String, Error> {
        self.client
            .run_utf8(format!(
                "attribute {} {} modifier remove {}",
                self.target.serialize(),
                self.attribute.serialize(),
                uuid.serialize(),
            ))
            .await
    }

    /// Returns the value of the modifier with the specified UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn modifier_value(self, uuid: Uuid, scale: Option<f64>) -> Result<String, Error> {
        let mut args = vec![
            Cow::Borrowed("attribute"),
            self.target.serialize(),
            self.attribute.serialize(),
            Cow::Borrowed("modifier"),
            Cow::Borrowed("value"),
            Cow::Borrowed("get"),
            uuid.serialize(),
        ];

        if let Some(scale) = scale {
            args.push(scale.serialize());
        }

        self.client.run_utf8(args.join(" ")).await
    }
}
