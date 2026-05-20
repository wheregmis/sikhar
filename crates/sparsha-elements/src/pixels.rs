//! Pixel helpers used by style refinements.

use std::fmt;

/// A CSS-pixel distance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixels(pub f32);

impl Pixels {
    pub const ZERO: Self = Self(0.0);

    pub fn value(self) -> f32 {
        self.0
    }
}

/// Construct a pixel distance.
pub const fn px(value: f32) -> Pixels {
    Pixels(value)
}

impl From<f32> for Pixels {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Pixels> for f32 {
    fn from(value: Pixels) -> Self {
        value.0
    }
}

impl fmt::Display for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}
