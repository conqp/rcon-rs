use std::borrow::Cow;
use std::collections::HashMap;

use uuid::Uuid;

/// Helper trait to serialize objects for minecraft command parameters.
pub trait Serialize {
    /// Serialize the object as a command parameter.
    fn serialize(&self) -> Cow<'_, str>;
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Cow<'_, str> {
        self.as_ref()
            .map_or(Cow::Borrowed(""), Serialize::serialize)
    }
}

impl<T> Serialize for &[T]
where
    T: Serialize,
{
    fn serialize(&self) -> Cow<'_, str> {
        let mut buffer = String::new();

        buffer.push('[');

        for (i, item) in self.iter().enumerate() {
            buffer.push_str(item.serialize().as_ref());

            if i < self.len().saturating_sub(1) {
                buffer.push(',');
            }
        }

        buffer.push(']');
        Cow::Owned(buffer)
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Cow<'_, str> {
        self.as_slice().serialize().into_owned().into()
    }
}

impl<K, V> Serialize for HashMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize(&self) -> Cow<'_, str> {
        let mut buffer = String::new();

        buffer.push('{');

        for (i, (key, value)) in self.iter().enumerate() {
            buffer.push_str(key.serialize().as_ref());
            buffer.push('=');
            buffer.push_str(value.serialize().as_ref());

            if i < self.len().saturating_sub(1) {
                buffer.push(',');
            }
        }

        buffer.push('}');
        buffer.into()
    }
}

impl Serialize for u64 {
    fn serialize(&self) -> Cow<'_, str> {
        self.to_string().into()
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Cow<'_, str> {
        self.to_string().into()
    }
}

impl Serialize for String {
    fn serialize(&self) -> Cow<'_, str> {
        self.into()
    }
}

impl Serialize for &str {
    fn serialize(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}

impl Serialize for Uuid {
    fn serialize(&self) -> Cow<'_, str> {
        self.to_string().into()
    }
}
