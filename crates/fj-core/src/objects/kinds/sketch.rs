use crate::{
    objects::{ObjectSet, Region},
    storage::Handle,
};

/// A 2-dimensional shape
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sketch {
    regions: ObjectSet<Region>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new(regions: impl IntoIterator<Item = Handle<Region>>) -> Self {
        Self {
            regions: regions.into_iter().collect(),
        }
    }

    /// Access the regions of the sketch
    pub fn regions(&self) -> &ObjectSet<Region> {
        &self.regions
    }
}
