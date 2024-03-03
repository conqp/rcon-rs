use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::to_server::ToServer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request<'passwd> {
    header: Header,
    password: &'passwd str,
}

impl<'passwd> Request<'passwd> {
    #[must_use]
    pub const fn new(header: Header, password: &'passwd str) -> Self {
        Self { header, password }
    }
}

impl<'passwd> From<&'passwd str> for Request<'passwd> {
    fn from(password: &'passwd str) -> Self {
        Self::new(Header::create(TYPE, password.as_bytes()), password)
    }
}

impl<'req, 'passwd> IntoIterator for &'req Request<'passwd> {
    type Item = u8;
    type IntoIter = <&'req Header as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.header.into_iter()
    }
}

impl ToServer for &Request<'_> {}
