//! Queries, adds, removes or sets an entity attribute.

use std::borrow::Cow;

pub use modifier::Modifier;
use uuid::Uuid;

use crate::minecraft::Serialize;
use crate::{Error, RCon};

mod modifier;

/// A proxy object to handle attribute-related commands.
#[derive(Debug)]
pub struct Proxy<'client, T> {
    client: &'client mut T,
    args: Vec<Cow<'client, str>>,
}

impl<'client, T> Proxy<'client, T> {
    pub(crate) const fn new(client: &'client mut T, args: Vec<Cow<'client, str>>) -> Self {
        Self { client, args }
    }
}

impl<T> Proxy<'_, T>
where
    T: RCon + Send,
{
    /// Returns the total value of the specified attribute.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn get(mut self, scale: Option<f64>) -> Result<String, Error> {
        self.args.push("get".into());

        if let Some(scale) = scale {
            self.args.push(Cow::Owned(scale.serialize().to_string()));
        }

        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Returns the base value of the specified attribute.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn base(mut self, scale: Option<f64>) -> Result<String, Error> {
        self.args.extend(["base".into(), "get".into()]);

        if let Some(scale) = scale {
            self.args.push(scale.serialize());
        }

        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Overwrites the base value of the specified attribute with the given value.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn set_base(mut self, value: f64) -> Result<String, Error> {
        self.args
            .extend(["base".into(), "get".into(), value.serialize()]);
        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Adds an attribute modifier with the specified properties
    /// if no modifier with the same UUID already existed.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn add_modifier<U>(
        mut self,
        uuid: Uuid,
        name: U,
        value: f64,
        modifier: Modifier,
    ) -> Result<String, Error>
    where
        U: AsRef<str>,
    {
        self.args.extend([
            "modifier".into(),
            "add".into(),
            uuid.serialize(),
            name.as_ref().into(),
            value.serialize(),
            modifier.serialize(),
        ]);
        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Removes the attribute modifier with the specified UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn remove_modifier(mut self, uuid: Uuid) -> Result<String, Error> {
        self.args
            .extend(["modifier".into(), "remove".into(), uuid.serialize()]);
        self.client.run_utf8(self.args.join(" ")).await
    }

    /// Returns the value of the modifier with the specified UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    pub async fn modifier_value(mut self, uuid: Uuid, scale: Option<f64>) -> Result<String, Error> {
        self.args.extend([
            "modifier".into(),
            "value".into(),
            "get".into(),
            uuid.serialize(),
        ]);

        if let Some(scale) = scale {
            self.args.push(scale.serialize());
        }

        self.client.run_utf8(self.args.join(" ")).await
    }
}
