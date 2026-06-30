use std::io::{Error, ErrorKind};

pub trait FromServer: Sized {
    fn is_valid(&self) -> bool;

    fn validate(self) -> std::io::Result<Self> {
        if self.is_valid() {
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid checksum."))
        }
    }
}
