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
        f: impl FnOnce(&Handle<Cycle>) -> Handle<Cycle>,
    ) -> Self;

    /// Add the provides interiors to the region
    #[must_use]
    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self;
}

impl UpdateRegion for Region {
    fn update_exterior(
        &self,
        f: impl FnOnce(&Handle<Cycle>) -> Handle<Cycle>,
    ) -> Self {
        let exterior = f(self.exterior());
        Region::new(exterior, self.interiors().iter().cloned(), self.color())
    }

    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self {
        let interiors = self.interiors().iter().cloned().chain(interiors);
        Region::new(self.exterior().clone(), interiors, self.color())
    }
}
