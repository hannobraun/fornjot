use crate::{
    objects::{Cycle, Region},
    operations::insert::Insert,
    storage::Handle,
    Instance,
};

/// Update a [`Region`]
pub trait UpdateRegion {
    /// Update the exterior of the region
    #[must_use]
    fn update_exterior<T>(
        &self,
        update: impl FnOnce(&Handle<Cycle>, &mut Instance) -> T,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>;

    /// Add the provided interiors to the region
    #[must_use]
    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self;

    /// Update an interior cycle of the region
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_interior<const N: usize>(
        &self,
        handle: &Handle<Cycle>,
        update: impl FnOnce(&Handle<Cycle>, &mut Instance) -> [Handle<Cycle>; N],
        core: &mut Instance,
    ) -> Self;
}

impl UpdateRegion for Region {
    fn update_exterior<T>(
        &self,
        update: impl FnOnce(&Handle<Cycle>, &mut Instance) -> T,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Cycle>>,
    {
        let exterior = update(self.exterior(), core).insert(&mut core.services);
        Region::new(exterior, self.interiors().iter().cloned(), self.color())
    }

    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self {
        let interiors = self.interiors().iter().cloned().chain(interiors);
        Region::new(self.exterior().clone(), interiors, self.color())
    }

    fn update_interior<const N: usize>(
        &self,
        handle: &Handle<Cycle>,
        update: impl FnOnce(&Handle<Cycle>, &mut Instance) -> [Handle<Cycle>; N],
        core: &mut Instance,
    ) -> Self {
        let interiors = self
            .interiors()
            .replace(handle, update(handle, core))
            .expect("Cycle not found");
        Region::new(self.exterior().clone(), interiors, self.color())
    }
}
