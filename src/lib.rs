//! A common interface for different `RCON` protocols.

use std::borrow::Cow;
use std::fmt::Debug;
use std::io::{Error, ErrorKind};

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
    /// # Returns
    ///
    /// Returns `true` if the login succeeded, otherwise `false`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred.
    fn login(&mut self, password: Cow<'_, str>) -> std::io::Result<bool>;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns the raw bytes from the server's response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred.
    fn run(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<Vec<u8>>;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns a valid UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred or if the returned bytes are not valid UTF-8.
    fn run_utf8(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<String> {
        self.run(args).and_then(|bytes| {
            String::from_utf8(bytes).map_err(|error| Error::new(ErrorKind::InvalidData, error))
        })
    }

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns a valid UTF-8 string that may be truncated.
    ///
    /// This command will not error when the returned bytes contain
    /// invalid UTF-8 bytes, but will replace them accordingly.
    ///
    /// See [`String::from_utf8_lossy`] for details.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred.
    fn run_utf8_lossy(&mut self, args: &[Cow<'_, str>]) -> std::io::Result<String> {
        self.run(args)
            .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
    }
}
