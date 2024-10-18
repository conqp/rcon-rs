use std::borrow::Cow;

use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    header: Header,
    password: String,
}

impl Request {
    #[must_use]
    pub const fn new(header: Header, password: String) -> Self {
        Self { header, password }
    }
}

impl<'cow, T> From<T> for Request
where
    T: Into<Cow<'cow, str>>,
{
    fn from(password: T) -> Self {
        let password = password.into();
        Self::new(
            Header::create(TYPE, password.as_bytes()),
            password.into_owned(),
        )
    }
}

impl IntoBytes for Request {
    fn into_bytes(self) -> impl AsRef<[u8]> {
        let header: [u8; Header::SIZE] = self.header.into();
        let password_bytes = self.password.as_bytes();
        let mut buffer = Vec::with_capacity(Header::SIZE + password_bytes.iter().len());
        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(password_bytes);
        buffer
    }
}
