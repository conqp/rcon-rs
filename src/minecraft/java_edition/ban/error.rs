use std::fmt::Display;

/// Errors that can occur when banning a player.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// An unknown selector has been specified.
    UnknownSelector,
    /// Another error occurred.
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::UnknownSelector => write!(f, "Unknown player"),
            Self::Other(error) => write!(f, "Unexpected error: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::UnknownSelector | Self::Other(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
