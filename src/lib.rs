//! A common interface for different `RCON` protocols.

use std::borrow::Cow;
use std::fmt::Debug;

#[cfg(feature = "battleye")]
pub mod battleye;
mod extensions;
#[cfg(feature = "source")]
pub mod source;

pub use extensions::*;

/// Common API for `RCON` protocol clients
pub trait RCon: Debug {
    /// Perform a login.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on errors.
    fn login(&mut self, password: Cow<'_, str>) -> std::io::Result<bool>;

    /// Run a command.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on errors.
    fn run(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<Vec<u8>>;
}

impl<'a, T> RCon for &'a mut T
where
    T: RCon,
{
    fn login(&mut self, password: Cow<'_, str>) -> std::io::Result<bool> {
        <T as RCon>::login(self, password)
    }

    fn run(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<Vec<u8>> {
        <T as RCon>::run(*self, args)
    }
}
