pub use ack::Ack;
pub use message::Message;

mod ack;
mod message;

pub const TYPE: u8 = 0x02;
