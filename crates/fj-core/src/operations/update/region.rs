use crate::{
    objects::{Cycle, Region},
    storage::Handle,
};

/// Update a [`Region`]
pub trait UpdateRegion {
    /// Update the exterior of the region
    #[must_use]
    fn update_exterior(
        &self,
        update: impl FnOnce(&Handle<Cycle>) -> Handle<Cycle>,
    ) -> Self;

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
        update: impl FnOnce(&Handle<Cycle>) -> [Handle<Cycle>; N],
    ) -> Self;
}

impl UpdateRegion for Region {
    fn update_exterior(
        &self,
        update: impl FnOnce(&Handle<Cycle>) -> Handle<Cycle>,
    ) -> Self {
        let exterior = update(self.exterior());
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
        update: impl FnOnce(&Handle<Cycle>) -> [Handle<Cycle>; N],
    ) -> Self {
        let interiors = self
            .interiors()
            .replace(handle, update(handle))
            .expect("Cycle not found");
        Region::new(self.exterior().clone(), interiors, self.color())
    }
}
