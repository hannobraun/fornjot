use crate::{
    Core,
    operations::{build::Polygon, derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::{Face, Region},
};

/// Update a [`Face`]
pub trait UpdateFace {
    /// Update the region of the face
    #[must_use]
    fn update_region<T>(
        &self,
        update: impl FnOnce(&Handle<Region>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>;
}

impl UpdateFace for Face {
    fn update_region<T>(
        &self,
        update: impl FnOnce(&Handle<Region>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
    {
        let region = update(self.region(), core);
        Face::new(
            self.surface().clone(),
            region.insert(core).derive_from(self.region(), core),
        )
    }
}

impl<const D: usize> UpdateFace for Polygon<D> {
    fn update_region<T>(
        &self,
        update: impl FnOnce(&Handle<Region>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
    {
        self.replace_face(self.face.update_region(update, core))
    }
}
