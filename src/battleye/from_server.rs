use std::io;

pub trait FromServer: Sized {
    fn is_valid(&self) -> bool;

    fn validate(self) -> io::Result<Self> {
        if self.is_valid() {
            Ok(self)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid checksum.",
            ))
        }
    }
}
