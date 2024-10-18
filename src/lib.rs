//! A common interface for different `RCON` protocols.

use std::borrow::Cow;

pub use utils::UdpSocketWrapper;

pub mod battleye;
pub mod source;
mod utils;

/// Common API for `RCON` protocol clients
pub trait RCon {
    /// Perform a login.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn login(&mut self, password: Cow<'_, str>) -> std::io::Result<bool>;

    /// Run a command.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn run(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<Vec<u8>>;
}
