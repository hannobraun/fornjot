use crate::{
    objects::{Region, Sketch},
    storage::Handle,
};

/// Update a [`Sketch`]
pub trait UpdateSketch {
    /// Add a region to the sketch
    fn add_region(&self, region: Handle<Region>) -> Self;
}

impl UpdateSketch for Sketch {
    fn add_region(&self, region: Handle<Region>) -> Self {
        Sketch::new(self.regions().cloned().chain([region]))
    }
}
