use num_derive::FromPrimitive;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum ServerData {
    ResponseValue = 0,
    ExecCommandOrAuthResponse = 2,
    Auth = 3,
}
