use crate::{
    objects::{Region, Set},
    storage::Handle,
};

/// A 2-dimensional shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    regions: Set<Region>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new(regions: impl IntoIterator<Item = Handle<Region>>) -> Self {
        Self {
            regions: regions.into_iter().collect(),
        }
    }

    /// Access the faces of the sketch
    pub fn regions(&self) -> &Set<Region> {
        &self.regions
    }
}
