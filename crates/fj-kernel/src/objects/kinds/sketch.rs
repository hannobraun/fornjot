use std::collections::BTreeSet;

use crate::{
    geometry::region::Region,
    objects::{FaceSet, Surface},
    services::Services,
    storage::Handle,
};

/// A 2-dimensional shape
///
/// # Implementation Note
///
/// The faces that make up the sketch must be in the same surface. This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    regions: BTreeSet<Region>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new(regions: impl IntoIterator<Item = Region>) -> Self {
        Self {
            regions: regions.into_iter().collect(),
        }
    }

    /// Apply the regions of the sketch to some [`Surface`]
    pub fn faces(
        &self,
        surface: Handle<Surface>,
        services: &mut Services,
    ) -> FaceSet {
        self.regions
            .iter()
            .map(|r| r.face(surface.clone(), services))
            .collect()
    }
}
