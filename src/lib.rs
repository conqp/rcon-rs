//! A common interface for different `RCON` protocols.

use std::future::Future;
use std::net::SocketAddr;

#[cfg(feature = "dayz")]
pub use dayz::DayZ;
pub use error::Error;
#[cfg(feature = "minecraft")]
pub use minecraft::Minecraft;

#[cfg(feature = "battleye")]
pub mod battleye;
#[cfg(feature = "dayz")]
pub mod dayz;
mod error;
#[cfg(feature = "minecraft")]
pub mod minecraft;
#[cfg(feature = "source")]
pub mod source;

/// Common API for `RCON` protocol clients
pub trait RCon {
    /// Create an `RCON` client by connecting to the specified address.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
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
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn login<T>(&mut self, password: T) -> impl Future<Output = std::io::Result<bool>> + Send
    where
        T: AsRef<[u8]> + Send;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns the raw bytes from the server's response.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn run<T>(&mut self, command: T) -> impl Future<Output = std::io::Result<Vec<u8>>> + Send
    where
        T: AsRef<[u8]> + Send;

    /// Run a command.
    ///
    /// # Returns
    ///
    /// Returns a valid UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O errors occurred or if the returned bytes are not valid UTF-8.
    fn run_utf8<T>(&mut self, command: T) -> impl Future<Output = Result<String, Error>> + Send
    where
        Self: Send,
        T: AsRef<[u8]> + Send,
    {
        async move { Ok(String::from_utf8(self.run(command).await?)?) }
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
    /// Returns an [`std::io::Error`] if any I/O errors occurred.
    fn run_utf8_lossy<T>(
        &mut self,
        command: T,
    ) -> impl Future<Output = std::io::Result<String>> + Send
    where
        Self: Send,
        T: AsRef<[u8]> + Send,
    {
        async move {
            self.run(command)
                .await
                .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        }
    }
}
