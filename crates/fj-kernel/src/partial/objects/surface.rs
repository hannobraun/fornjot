use crate::{
    geometry::surface::SurfaceGeometry,
    objects::{Objects, Surface},
    partial::MergeWith,
    validate::ValidationError,
};

/// A partial [`Surface`]
///
/// See [`crate::partial`] for more information
#[derive(Clone, Debug, Default)]
pub struct PartialSurface {
    /// The geometry that defines the [`Surface`]
    pub geometry: Option<SurfaceGeometry>,
}

impl PartialSurface {
    /// Build a full [`Surface`] from the partial surface
    pub fn build(self, _: &Objects) -> Result<Surface, ValidationError> {
        let geometry = self
            .geometry
            .expect("Can't build `Surface` without geometry");

        Ok(Surface::new(geometry))
    }
}

impl MergeWith for PartialSurface {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            geometry: self.geometry.merge_with(other.geometry),
        }
    }
}

impl From<&Surface> for PartialSurface {
    fn from(surface: &Surface) -> Self {
        Self {
            geometry: Some(surface.geometry()),
        }
    }
}
