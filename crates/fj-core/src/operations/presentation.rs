//! Operations to control the presentation of objects

use fj_interop::Color;

use crate::{
    objects::{IsObject, Region},
    storage::Handle,
    Core,
};

/// Get the color of an object
pub trait GetColor {
    /// Get the color of the object
    fn get_color(&self, core: &mut Core) -> Option<Color>;
}

impl GetColor for Handle<Region> {
    fn get_color(&self, core: &mut Core) -> Option<Color> {
        core.layers.presentation.color.get(self).copied()
    }
}

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
        core: &mut Core,
    ) -> Self::BareObject {
        let color = color.into();

        core.layers.presentation.set_color(self.clone(), color);

        Region::new(
            self.exterior().clone(),
            self.interiors().into_iter().cloned(),
            Some(color),
        )
    }
}
