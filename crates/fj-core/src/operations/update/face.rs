use crate::{
    objects::{Face, Region},
    operations::Polygon,
    storage::Handle,
};

/// Update a [`Face`]
pub trait UpdateFace {
    /// Replace the region of the face
    #[must_use]
    fn update_region(
        &self,
        f: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self;
}

impl UpdateFace for Face {
    fn update_region(
        &self,
        f: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self {
        let region = f(self.region());
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> UpdateFace for Polygon<D> {
    fn update_region(
        &self,
        f: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self {
        self.replace_face(self.face.update_region(f))
    }
}
