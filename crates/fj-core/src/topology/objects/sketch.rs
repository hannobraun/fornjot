use crate::{
    storage::Handle,
    topology::{ObjectSet, Region, Surface},
};

/// A 2-dimensional shape
#[derive(Clone, Debug)]
pub struct Sketch {
    surface: Handle<Surface>,
    regions: ObjectSet<Region>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new(
        surface: Handle<Surface>,
        regions: impl IntoIterator<Item = Handle<Region>>,
    ) -> Self {
        Self {
            surface,
            regions: regions.into_iter().collect(),
        }
    }

    /// Access the surface of the sketch
    pub fn surface(&self) -> &Handle<Surface> {
        &self.surface
    }

    /// Access the regions of the sketch
    pub fn regions(&self) -> &ObjectSet<Region> {
        &self.regions
    }
}
