use std::borrow::Cow;
use std::collections::BTreeMap;

use uuid::Uuid;

/// Helper trait to serialize objects for minecraft command parameters.
pub trait Serialize {
    /// Serialize the object as a command parameter.
    fn serialize(self) -> Cow<'static, str>;
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize(self) -> Cow<'static, str> {
        self.map_or(Cow::Borrowed(""), Serialize::serialize)
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize(self) -> Cow<'static, str> {
        let mut buffer = String::new();
        let len = self.len();

        buffer.push('[');

        for (i, item) in self.into_iter().enumerate() {
            buffer.push_str(item.serialize().as_ref());

            if i < len.saturating_sub(1) {
                buffer.push(',');
            }
        }

        buffer.push(']');
        Cow::Owned(buffer)
    }
}

impl<K, V> Serialize for BTreeMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize(self) -> Cow<'static, str> {
        let mut buffer = String::new();
        let len = self.len();
        buffer.push('{');

        for (i, (key, value)) in self.into_iter().enumerate() {
            buffer.push_str(key.serialize().as_ref());
            buffer.push('=');
            buffer.push_str(value.serialize().as_ref());

            if i < len.saturating_sub(1) {
                buffer.push(',');
            }
        }

        buffer.push('}');
        Cow::Owned(buffer)
    }
}

impl Serialize for u64 {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}

impl Serialize for f64 {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}

impl Serialize for String {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(self)
    }
}

impl Serialize for &'static str {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Borrowed(self)
    }
}

impl Serialize for Cow<'static, str> {
    fn serialize(self) -> Cow<'static, str> {
        self
    }
}

impl Serialize for Uuid {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
