use crate::{
    geometry::surface::SurfaceGeometry,
    objects::{Objects, Surface},
    partial::MergeWith,
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
    pub fn build(self, _: &Objects) -> Surface {
        let geometry = self
            .geometry
            .expect("Can't build `Surface` without geometry");

        Surface::new(geometry)
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
