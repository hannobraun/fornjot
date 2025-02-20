use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::{Curve, HalfEdge, Vertex},
};

/// Update a [`HalfEdge`]
pub trait UpdateHalfEdge {
    /// Update the curve of the edge
    #[must_use]
    fn update_curve<T>(
        &self,
        update: impl FnOnce(&Handle<Curve>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Curve>>;

    /// Update the start vertex of the edge
    #[must_use]
    fn update_start_vertex<T>(
        &self,
        update: impl FnOnce(&Handle<Vertex>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Vertex>>;
}

impl UpdateHalfEdge for HalfEdge {
    fn update_curve<T>(
        &self,
        update: impl FnOnce(&Handle<Curve>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Curve>>,
    {
        HalfEdge::new(
            update(self.curve(), core)
                .insert(core)
                .derive_from(self.curve(), core),
            self.start_vertex().clone(),
        )
    }

    fn update_start_vertex<T>(
        &self,
        update: impl FnOnce(&Handle<Vertex>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Vertex>>,
    {
        HalfEdge::new(
            self.curve().clone(),
            update(self.start_vertex(), core)
                .insert(core)
                .derive_from(self.start_vertex(), core),
        )
    }
}
