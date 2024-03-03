use super::TYPE;
use crate::battleye::header::Header;
use crate::battleye::to_server::ToServer;
use std::array::IntoIter;
use std::iter::{Chain, Copied};
use std::slice::Iter;

const SEQ: u8 = 0x00;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request<'cmd> {
    header: Header,
    seq: u8,
    command: &'cmd str,
}

impl<'cmd> Request<'cmd> {
    #[must_use]
    pub const fn new(header: Header, seq: u8, command: &'cmd str) -> Self {
        Self {
            header,
            seq,
            command,
        }
    }
}

impl<'cmd> From<&'cmd str> for Request<'cmd> {
    fn from(command: &'cmd str) -> Self {
        Self::new(
            Header::create(
                TYPE,
                SEQ.to_le_bytes()
                    .into_iter()
                    .chain(command.as_bytes().iter().copied())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            SEQ,
            command,
        )
    }
}

impl<'req, 'cmd> IntoIterator for &'req Request<'cmd> {
    type Item = u8;
    type IntoIter = Chain<
        Chain<<&'req Header as IntoIterator>::IntoIter, IntoIter<Self::Item, 1>>,
        Copied<Iter<'cmd, Self::Item>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.header
            .into_iter()
            .chain(self.seq.to_le_bytes())
            .chain(self.command.as_bytes().iter().copied())
    }
}

impl ToServer for &Request<'_> {}
