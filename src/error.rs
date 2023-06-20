use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    NotLoggedIn,
    InvalidServerData(i32),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotLoggedIn => write!(f, "not logged in"),
            Self::InvalidServerData(number) => write!(f, "invalid server data: {}", number),
        }
    }
}
