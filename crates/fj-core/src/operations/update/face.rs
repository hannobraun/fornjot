use crate::{
    Core,
    operations::{
        build::Polygon, derive::DeriveFrom, insert::Insert, selector::Selector,
    },
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

/// Update a [`Face`] with flexible selectors
///
/// This trait provides a more flexible interface for updating faces, allowing
/// objects to be selected using the `Selector` trait.
pub trait UpdateFaceWithSelector {
    /// Update regions selected by the given selector
    #[must_use]
    fn update_regions<T>(
        &self,
        selector: impl Selector<Region>,
        update: impl Fn(&Handle<Region>, &mut Core) -> T,
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

impl UpdateFaceWithSelector for Face {
    fn update_regions<T>(
        &self,
        selector: impl Selector<Region>,
        update: impl Fn(&Handle<Region>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
    {
        // For Face, there's only one region, so the selector selects from a single-item set
        use crate::topology::ObjectSet;
        let region_set = ObjectSet::new([self.region().clone()]);
        let selected_handles: Vec<_> = selector.select(&region_set).collect();

        if let Some(region_handle) = selected_handles.first() {
            let updated_region = update(region_handle, core);
            Face::new(
                self.surface().clone(),
                updated_region.insert(core).derive_from(self.region(), core),
            )
        } else {
            self.clone()
        }
    }
}

impl<const D: usize> UpdateFaceWithSelector for Polygon<D> {
    fn update_regions<T>(
        &self,
        selector: impl Selector<Region>,
        update: impl Fn(&Handle<Region>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
    {
        self.replace_face(self.face.update_regions(selector, update, core))
    }
}
