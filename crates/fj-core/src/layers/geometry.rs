//! Layer infrastructure for [`Geometry`]

use crate::{
    geometry::{Geometry, SurfaceGeometry},
    objects::Surface,
    storage::Handle,
};

use super::{Command, Event, Layer};

impl Layer<Geometry> {
    /// Define the geometry of the provided surface
    pub fn define_surface(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGeometry,
    ) {
        let mut events = Vec::new();
        self.process(DefineSurface { surface, geometry }, &mut events);
    }
}

/// Define the geometry of a surface
pub struct DefineSurface {
    surface: Handle<Surface>,
    geometry: SurfaceGeometry,
}

impl Command<Geometry> for DefineSurface {
    type Result = ();
    type Event = Self;

    fn decide(
        self,
        _: &Geometry,
        events: &mut Vec<Self::Event>,
    ) -> Self::Result {
        events.push(self);
    }
}

impl Event<Geometry> for DefineSurface {
    fn evolve(&self, state: &mut Geometry) {
        state.define_surface_inner(self.surface.clone(), self.geometry);
    }
}
