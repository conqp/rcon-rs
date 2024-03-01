use async_std::net::ToSocketAddrs;
use std::io;
use std::sync::Arc;
use std::time::Duration;

pub mod source;

pub trait RCon: Sized {
    /// Connect to the given socket address.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn connect<T>(address: T) -> impl std::future::Future<Output = io::Result<Self>> + Send
    where
        T: ToSocketAddrs + Send + Sync,
        <T as ToSocketAddrs>::Iter: Send + Sync;

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
    fn run<T>(
        &mut self,
        args: &[T],
        multi_packet_timeout: Option<Duration>,
    ) -> impl std::future::Future<Output = io::Result<Arc<[u8]>>> + Send
    where
        T: AsRef<str> + Send + Sync;
}
