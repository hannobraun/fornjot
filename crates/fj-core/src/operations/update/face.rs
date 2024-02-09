use crate::{
    objects::{Face, Region},
    operations::build::Polygon,
    storage::Handle,
    Instance,
};

/// Update a [`Face`]
pub trait UpdateFace {
    /// Update the region of the face
    #[must_use]
    fn update_region(
        &self,
        update: impl FnOnce(&Handle<Region>, &mut Instance) -> Handle<Region>,
        core: &mut Instance,
    ) -> Self;
}

impl UpdateFace for Face {
    fn update_region(
        &self,
        update: impl FnOnce(&Handle<Region>, &mut Instance) -> Handle<Region>,
        core: &mut Instance,
    ) -> Self {
        let region = update(self.region(), core);
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> UpdateFace for Polygon<D> {
    fn update_region(
        &self,
        update: impl FnOnce(&Handle<Region>, &mut Instance) -> Handle<Region>,
        core: &mut Instance,
    ) -> Self {
        self.replace_face(self.face.update_region(update, core))
    }
}
