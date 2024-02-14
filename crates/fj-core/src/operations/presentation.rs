//! Operations to control the presentation of objects

use fj_interop::Color;

use crate::{
    objects::{IsObject, Region},
    storage::Handle,
    Core,
};

/// Set the color of an object
pub trait SetColor: IsObject {
    /// Set the color of the object
    fn set_color(
        &self,
        color: impl Into<Color>,
        core: &mut Core,
    ) -> Self::BareObject;
}

impl SetColor for Handle<Region> {
    fn set_color(
        &self,
        color: impl Into<Color>,
        _core: &mut Core,
    ) -> Self::BareObject {
        let color = color.into();

        Region::new(
            self.exterior().clone(),
            self.interiors().into_iter().cloned(),
            Some(color),
        )
    }
}
