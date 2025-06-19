use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, selector::Selector},
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

/// Update a [`Sketch`] with flexible selectors
///
/// This trait provides a more flexible interface for updating sketches, allowing
/// objects to be selected using the `Selector` trait.
pub trait UpdateSketchWithSelector {
    /// Add a region to the sketch
    #[must_use]
    fn add_regions<T>(
        &self,
        regions: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>;

    /// Update regions selected by the given selector
    ///
    /// # Panics
    ///
    /// Panics, if any selected object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_regions<T, R>(
        &self,
        selector: impl Selector<Region>,
        update: impl Fn(&Handle<Region>, &mut Core) -> R,
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

impl UpdateSketchWithSelector for Sketch {
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

    fn update_regions<T, R>(
        &self,
        selector: impl Selector<Region>,
        update: impl Fn(&Handle<Region>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
        R: IntoIterator<Item = T>,
    {
        let selected_handles: Vec<_> =
            selector.select(self.regions()).collect();

        let mut result = self.clone();
        for handle in selected_handles {
            result = result.update_region(handle, &update, core);
        }
        result
    }
}
