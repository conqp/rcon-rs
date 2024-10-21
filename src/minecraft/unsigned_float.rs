use std::borrow::Cow;

use crate::minecraft::serialize::Serialize;

/// A float value that is required to be non-negative.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct UnsignedFloat(f64);

impl Serialize for UnsignedFloat {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(self.0.to_string())
    }
}

impl From<UnsignedFloat> for f64 {
    fn from(unsigned: UnsignedFloat) -> f64 {
        unsigned.0
    }
}

impl TryFrom<f64> for UnsignedFloat {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_sign_negative() {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}
