use crate::{
    objects::{Cycle, Region},
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    Core,
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
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_interior<T, const N: usize>(
        &self,
        handle: &Handle<Cycle>,
        update: impl FnOnce(&Handle<Cycle>, &mut Core) -> [T; N],
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>;
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
        Region::new(exterior, self.interiors().iter().cloned(), self.color())
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
        Region::new(self.exterior().clone(), interiors, self.color())
    }

    fn update_interior<T, const N: usize>(
        &self,
        handle: &Handle<Cycle>,
        update: impl FnOnce(&Handle<Cycle>, &mut Core) -> [T; N],
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
    {
        let interiors = self
            .interiors()
            .replace(
                handle,
                update(handle, core).map(|object| {
                    object.insert(core).derive_from(handle, core)
                }),
            )
            .expect("Cycle not found");
        Region::new(self.exterior().clone(), interiors, self.color())
    }
}
