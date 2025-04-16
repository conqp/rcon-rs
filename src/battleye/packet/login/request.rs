use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::into_bytes::IntoBytes;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    header: Header,
    password: Vec<u8>,
}

impl Request {
    #[must_use]
    pub const fn new(header: Header, password: Vec<u8>) -> Self {
        Self { header, password }
    }
}

impl From<&[u8]> for Request {
    fn from(password: &[u8]) -> Self {
        Self::new(Header::create(TYPE, password), password.to_vec())
    }
}

impl IntoBytes for Request {
    fn into_bytes(self) -> impl AsRef<[u8]> {
        let header: [u8; Header::SIZE] = self.header.into();
        let password_bytes = self.password;
        let mut buffer = Vec::with_capacity(Header::SIZE + password_bytes.iter().len());
        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(&password_bytes);
        buffer
    }
}
