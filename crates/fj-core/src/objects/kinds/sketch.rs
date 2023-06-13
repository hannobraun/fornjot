use std::collections::BTreeSet;

use crate::{
    objects::{FaceSet, Region, Surface},
    services::Services,
    storage::Handle,
};

/// A 2-dimensional shape
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

    /// Access the regions of the sketch
    pub fn regions(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter()
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
