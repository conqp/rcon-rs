//! A common interface for different `RCON` protocols.

use std::future::Future;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;

#[cfg(feature = "battleye")]
pub mod battleye;
mod extensions;
#[cfg(feature = "source")]
pub mod source;

pub use extensions::*;

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
    fn login(&mut self, password: &str) -> impl Future<Output = std::io::Result<bool>> + Send;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns the raw bytes from the server's response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred.
    fn run<T>(&mut self, args: &[T]) -> impl Future<Output = std::io::Result<Vec<u8>>> + Send
    where
        T: AsRef<str> + Send + Sync;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns a valid UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred or if the returned bytes are not valid UTF-8.
    fn run_utf8<T>(&mut self, args: &[T]) -> impl Future<Output = std::io::Result<String>> + Send
    where
        Self: Send,
        T: AsRef<str> + Send + Sync,
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
    fn run_utf8_lossy<T>(
        &mut self,
        args: &[T],
    ) -> impl Future<Output = std::io::Result<String>> + Send
    where
        Self: Send,
        T: AsRef<str> + Send + Sync,
    {
        async {
            self.run(args)
                .await
                .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        }
    }
}
