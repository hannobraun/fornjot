use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::{Region, Sketch},
};

/// Update a [`Sketch`]
pub trait UpdateSketch {
    /// Add a region to the sketch
    #[must_use]
    fn add_regions<T>(
        &self,
        regions: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>;

    /// Update a region of the sketch
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_region<T, R>(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
        R: IntoIterator<Item = T>;
}

impl UpdateSketch for Sketch {
    fn add_regions<T>(
        &self,
        regions: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
    {
        let regions = regions.into_iter().map(|region| region.insert(core));
        let regions = self.regions().iter().cloned().chain(regions);
        Sketch::new(self.surface().clone(), regions)
    }

    fn update_region<T, R>(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
        R: IntoIterator<Item = T>,
    {
        let regions = self
            .regions()
            .replace(
                handle,
                update(handle, core).into_iter().map(|object| {
                    object.insert(core).derive_from(handle, core)
                }),
            )
            .expect("Region not found");
        Sketch::new(self.surface().clone(), regions)
    }
}
