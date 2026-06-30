use num_derive::FromPrimitive;

/// Server data message types.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum ServerData {
    /// A `ResponseValue` message.
    ResponseValue = 0,

    /// An `ExecCommand` or `AuthResponse` message.
    ExecCommandOrAuthResponse = 2,

    /// An `Auth` message.
    Auth = 3,
}
