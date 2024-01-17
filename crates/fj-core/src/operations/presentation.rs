//! Operations to control the presentation of objects

use fj_interop::Color;

/// Set the color of an object
pub trait SetColor {
    /// Set the color of the object
    fn set_color(&self, color: impl Into<Color>) -> Self;
}
