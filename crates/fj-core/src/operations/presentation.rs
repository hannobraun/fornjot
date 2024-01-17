//! Operations to control the presentation of objects

use fj_interop::Color;

use crate::objects::Region;

/// Set the color of an object
pub trait SetColor {
    /// Set the color of the object
    fn set_color(&self, color: impl Into<Color>) -> Self;
}

impl SetColor for Region {
    fn set_color(&self, color: impl Into<Color>) -> Self {
        Region::new(
            self.exterior().clone(),
            self.interiors().into_iter().cloned(),
            Some(color.into()),
        )
    }
}
