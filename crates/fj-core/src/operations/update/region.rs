use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, selector::Selector},
    storage::Handle,
    topology::{Cycle, Region},
};

/// Update a [`Region`]
pub trait UpdateRegion {
    /// Update the exterior of the region
    #[must_use]
    fn update_exterior<T>(
        &self,
        update: impl FnOnce(&Handle<Cycle>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>;

    /// Add the provided interiors to the region
    #[must_use]
    fn add_interiors<T>(
        &self,
        interiors: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>;

    /// Update an interior cycle of the region
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_interior<T, R>(
        &self,
        handle: &Handle<Cycle>,
        update: impl FnOnce(&Handle<Cycle>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
        R: IntoIterator<Item = T>;
}

/// Update a [`Region`] with flexible selectors
///
/// This trait provides a more flexible interface for updating regions, allowing
/// objects to be selected using the `Selector` trait.
pub trait UpdateRegionWithSelector {
    /// Update exteriors selected by the given selector
    #[must_use]
    fn update_exteriors<T>(
        &self,
        selector: impl Selector<Cycle>,
        update: impl Fn(&Handle<Cycle>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>;

    /// Add the provided interiors to the region
    #[must_use]
    fn add_interiors<T>(
        &self,
        interiors: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>;

    /// Update interior cycles selected by the given selector
    ///
    /// # Panics
    ///
    /// Panics, if any selected object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_interiors<T, R>(
        &self,
        selector: impl Selector<Cycle>,
        update: impl Fn(&Handle<Cycle>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
        R: IntoIterator<Item = T>;
}

impl UpdateRegion for Region {
    fn update_exterior<T>(
        &self,
        update: impl FnOnce(&Handle<Cycle>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
    {
        let exterior = update(self.exterior(), core)
            .insert(core)
            .derive_from(self.exterior(), core);
        Region::new(exterior, self.interiors().iter().cloned())
    }

    fn add_interiors<T>(
        &self,
        interiors: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
    {
        let interiors = interiors.into_iter().map(|cycle| cycle.insert(core));
        let interiors = self.interiors().iter().cloned().chain(interiors);
        Region::new(self.exterior().clone(), interiors)
    }

    fn update_interior<T, R>(
        &self,
        handle: &Handle<Cycle>,
        update: impl FnOnce(&Handle<Cycle>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
        R: IntoIterator<Item = T>,
    {
        let interiors = self
            .interiors()
            .replace(
                handle,
                update(handle, core).into_iter().map(|object| {
                    object.insert(core).derive_from(handle, core)
                }),
            )
            .expect("Cycle not found");
        Region::new(self.exterior().clone(), interiors)
    }
}

impl UpdateRegionWithSelector for Region {
    fn update_exteriors<T>(
        &self,
        selector: impl Selector<Cycle>,
        update: impl Fn(&Handle<Cycle>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
    {
        // For Region, there's only one exterior, so the selector selects from a single-item set
        use crate::topology::ObjectSet;
        let exterior_set = ObjectSet::new([self.exterior().clone()]);
        let selected_handles: Vec<_> = selector.select(&exterior_set).collect();

        if let Some(exterior_handle) = selected_handles.first() {
            let updated_exterior = update(exterior_handle, core);
            Region::new(
                updated_exterior
                    .insert(core)
                    .derive_from(self.exterior(), core),
                self.interiors().iter().cloned(),
            )
        } else {
            self.clone()
        }
    }

    fn add_interiors<T>(
        &self,
        interiors: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
    {
        let interiors = interiors.into_iter().map(|cycle| cycle.insert(core));
        let interiors = self.interiors().iter().cloned().chain(interiors);
        Region::new(self.exterior().clone(), interiors)
    }

    fn update_interiors<T, R>(
        &self,
        selector: impl Selector<Cycle>,
        update: impl Fn(&Handle<Cycle>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
        R: IntoIterator<Item = T>,
    {
        let selected_handles: Vec<_> =
            selector.select(self.interiors()).collect();

        let mut result = self.clone();
        for handle in selected_handles {
            result = result.update_interior(handle, &update, core);
        }
        result
    }
}
