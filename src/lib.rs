//! A common interface for different `RCON` protocols.

use std::borrow::Cow;

#[cfg(feature = "battleye")]
pub mod battleye;
mod extensions;
#[cfg(feature = "source")]
pub mod source;

#[cfg(feature = "dayz")]
pub use extensions::dayz;

pub use extensions::{Broadcast, Say};

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
