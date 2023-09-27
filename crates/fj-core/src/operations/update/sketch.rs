use crate::{
    objects::{Region, Sketch},
    storage::Handle,
};

/// Update a [`Sketch`]
pub trait UpdateSketch {
    /// Add a region to the sketch
    #[must_use]
    fn add_region(&self, region: Handle<Region>) -> Self;

    /// Update a region of the sketch
    ///
    /// # Panics
    ///
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn update_region(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self;
}

impl UpdateSketch for Sketch {
    fn add_region(&self, region: Handle<Region>) -> Self {
        Sketch::new(self.regions().iter().cloned().chain([region]))
    }

    fn update_region(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self {
        let regions = self.regions().update(handle, update);
        Sketch::new(regions)
    }
}
