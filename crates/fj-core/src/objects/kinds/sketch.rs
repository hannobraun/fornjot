use crate::{
    objects::{handles::Handles, Region},
    storage::Handle,
};

/// A 2-dimensional shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    regions: Handles<Region>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new(regions: impl IntoIterator<Item = Handle<Region>>) -> Self {
        Self {
            regions: regions.into_iter().collect(),
        }
    }

    /// Access the regions of the sketch
    pub fn regions(&self) -> &Handles<Region> {
        &self.regions
    }
}
