use std::borrow::Cow;
use std::time::Duration;

use crate::minecraft::Serialize;

/// Fading times.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Time {
    fade_in: Duration,
    hold: Duration,
    fade_out: Duration,
}

impl Time {
    /// Create a new `FadeTime`.
    #[must_use]
    pub const fn new(fade_in: Duration, hold: Duration, fade_out: Duration) -> Self {
        Self {
            fade_in,
            hold,
            fade_out,
        }
    }

    /// Return the fade-in duration.
    #[must_use]
    pub const fn fade_in(self) -> Duration {
        self.fade_in
    }

    /// Return the hold duration.
    #[must_use]
    pub const fn hold(self) -> Duration {
        self.hold
    }

    /// Return the fade-out duration.
    #[must_use]
    pub const fn fade_out(self) -> Duration {
        self.fade_out
    }
}

impl Serialize for Time {
    fn serialize(self) -> Cow<'static, str> {
        Cow::Owned(format!(
            "time {} {} {}",
            self.fade_in.as_secs_f32(),
            self.hold.as_secs_f32(),
            self.fade_out.as_secs_f32(),
        ))
    }
}
