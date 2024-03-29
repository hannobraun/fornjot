use crate::{
    storage::Handle,
    topology::{ObjectSet, Region},
};

/// A 2-dimensional shape
#[derive(Clone, Debug)]
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
