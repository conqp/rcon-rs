use super::TYPE;
use crate::battleye::header::Header;

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

impl From<Request> for Box<[u8]> {
    fn from(request: Request) -> Box<[u8]> {
        let header: [u8; Header::SIZE] = request.header.into();
        let password_bytes = request.password;
        let mut buffer = Vec::with_capacity(Header::SIZE + password_bytes.iter().len());
        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(&password_bytes);
        buffer.into_boxed_slice()
    }
}
