use std::io;
use std::sync::Arc;

pub mod source;

pub trait RCon {
    /// Perform a login.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn login(&mut self, password: &str) -> io::Result<bool>;

    /// Run a command.
    ///
    /// # Errors
    /// Returns an [`io::Error`] on errors.
    fn run<T>(&mut self, args: &[T]) -> io::Result<Vec<u8>>
    where
        T: AsRef<str>;
}

pub trait AsyncRCon {
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
    ) -> impl std::future::Future<Output = io::Result<Arc<[u8]>>> + Send
    where
        T: AsRef<str> + Send + Sync;
}
