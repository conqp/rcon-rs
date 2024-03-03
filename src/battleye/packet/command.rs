mod request;
mod response;

pub const TYPE: u8 = 0x01;
pub use request::Request;
pub use response::Response;
