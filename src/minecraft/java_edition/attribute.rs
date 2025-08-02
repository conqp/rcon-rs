//! Queries, adds, removes or sets an entity attribute.

use std::future::Future;

pub use modifier::Modifier;
use uuid::Uuid;

use crate::minecraft::proxy::Proxy;
use crate::minecraft::{Error, Serialize};
use crate::RCon;

mod modifier;

/// Attribute-related operations.
pub trait Attribute {
    /// Returns the total value of the specified attribute.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn get(&mut self, scale: Option<f64>) -> impl Future<Output = Result<String, Error>> + Send;

    /// Returns the base value of the specified attribute.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn base(&mut self, scale: Option<f64>) -> impl Future<Output = Result<String, Error>> + Send;

    /// Overwrites the base value of the specified attribute with the given value.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn set_base(&mut self, value: f64) -> impl Future<Output = Result<String, Error>> + Send;

    /// Adds an attribute modifier with the specified properties
    /// if no modifier with the same UUID already existed.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn add_modifier<U>(
        &mut self,
        uuid: Uuid,
        name: U,
        value: f64,
        modifier: Modifier,
    ) -> impl Future<Output = Result<String, Error>> + Send
    where
        U: ToString + Send;

    /// Removes the attribute modifier with the specified UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn remove_modifier(&mut self, uuid: Uuid)
        -> impl Future<Output = Result<String, Error>> + Send;

    /// Returns the value of the modifier with the specified UUID.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn modifier_value(
        &mut self,
        uuid: Uuid,
        scale: Option<f64>,
    ) -> impl Future<Output = Result<String, Error>> + Send;
}

impl<T> Attribute for Proxy<'_, T>
where
    T: RCon + Send,
{
    async fn get(&mut self, scale: Option<f64>) -> Result<String, Error> {
        let mut args = vec!["get".into()];

        if let Some(scale) = scale {
            args.push(scale.serialize());
        }

        self.run_utf8(&args).await
    }

    async fn base(&mut self, scale: Option<f64>) -> Result<String, Error> {
        let mut args = vec!["base".into(), "get".into()];

        if let Some(scale) = scale {
            args.push(scale.serialize());
        }

        self.run_utf8(&args).await
    }

    async fn set_base(&mut self, value: f64) -> Result<String, Error> {
        self.run_utf8(&["base".into(), "get".into(), value.serialize()])
            .await
    }

    async fn add_modifier<U>(
        &mut self,
        uuid: Uuid,
        name: U,
        value: f64,
        modifier: Modifier,
    ) -> Result<String, Error>
    where
        U: ToString + Send,
    {
        self.run_utf8(&[
            "modifier".into(),
            "add".into(),
            uuid.serialize(),
            name.to_string().serialize(),
            value.serialize(),
            modifier.serialize(),
        ])
        .await
    }

    async fn remove_modifier(&mut self, uuid: Uuid) -> Result<String, Error> {
        self.run_utf8(&["modifier".into(), "remove".into(), uuid.serialize()])
            .await
    }

    async fn modifier_value(&mut self, uuid: Uuid, scale: Option<f64>) -> Result<String, Error> {
        let mut args = vec![
            "modifier".into(),
            "value".into(),
            "get".into(),
            uuid.serialize(),
        ];

        if let Some(scale) = scale {
            args.push(scale.serialize());
        }

        self.run_utf8(&args).await
    }
}
