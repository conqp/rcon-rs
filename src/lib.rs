//! A common interface for different `RCON` protocols.

use std::borrow::Cow;
use std::io;
use tokio::net::ToSocketAddrs;

pub mod battleye;
pub mod source;

/// Common API for `RCON` protocol clients
pub trait RCon: Sized {
    /// Connect to the given socket address.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn connect<T>(address: T) -> impl std::future::Future<Output = io::Result<Self>> + Send
    where
        T: ToSocketAddrs + Send;

    /// Perform a login.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn login(
        &mut self,
        password: &str,
    ) -> impl std::future::Future<Output = io::Result<bool>> + Send;

    /// Run a command.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn run<'a>(
        &mut self,
        args: &[Cow<'a, str>],
    ) -> impl std::future::Future<Output = io::Result<Vec<u8>>> + Send;
}
