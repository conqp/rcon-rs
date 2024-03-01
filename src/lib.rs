use std::io;

pub mod source;

pub trait Rcon {
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
