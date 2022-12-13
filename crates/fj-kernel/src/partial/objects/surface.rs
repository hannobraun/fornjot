use crate::{
    geometry::surface::SurfaceGeometry,
    objects::{Objects, Surface},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
};

/// A partial [`Surface`]
#[derive(Clone, Debug)]
pub struct PartialSurface {
    /// The surface's geometry
    pub geometry: Option<SurfaceGeometry>,
}

impl PartialSurface {
    /// Construct an instance of `PartialSurface`
    pub fn new() -> Self {
        Self { geometry: None }
    }
}

impl PartialObject for PartialSurface {
    type Full = Surface;

    fn from_full(surface: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            geometry: Some(surface.geometry()),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let geometry = self
            .geometry
            .expect("Can't build `Surface` without geometry");

        Surface::new(geometry)
    }
}

impl Default for PartialSurface {
    fn default() -> Self {
        Self::new()
    }
}
