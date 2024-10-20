//! A common interface for different `RCON` protocols.

use std::borrow::Cow;
use std::future::Future;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;

#[cfg(feature = "battleye")]
pub mod battleye;
#[cfg(feature = "dayz")]
pub mod dayz;
#[cfg(feature = "source")]
pub mod source;
#[cfg(feature = "dayz")]
pub use crate::dayz::DayZ;

/// Common API for `RCON` protocol clients
pub trait RCon {
    /// Create an `RCON` client by connecting to the specified address.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred.
    fn connect<T>(address: T) -> impl Future<Output = std::io::Result<Self>>
    where
        Self: Sized,
        T: Into<SocketAddr> + Send;

    /// Perform a login.
    ///
    /// # Returns
    ///
    /// Returns `true` if the login succeeded, otherwise `false`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred.
    fn login(
        &mut self,
        password: Cow<'_, str>,
    ) -> impl Future<Output = std::io::Result<bool>> + Send;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns the raw bytes from the server's response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred.
    fn run(
        &mut self,
        args: &[Cow<'_, str>],
    ) -> impl Future<Output = std::io::Result<Vec<u8>>> + Send;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns a valid UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred or if the returned bytes are not valid UTF-8.
    fn run_utf8(
        &mut self,
        args: &[Cow<'_, str>],
    ) -> impl Future<Output = std::io::Result<String>> + Send
    where
        Self: Send,
    {
        async {
            self.run(args).await.and_then(|bytes| {
                String::from_utf8(bytes).map_err(|error| Error::new(ErrorKind::InvalidData, error))
            })
        }
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
    fn run_utf8_lossy(
        &mut self,
        args: &[Cow<'_, str>],
    ) -> impl Future<Output = std::io::Result<String>> + Send
    where
        Self: Send,
    {
        async {
            self.run(args)
                .await
                .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        }
    }
}
