use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, selector::Selector},
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

/// Update a [`HalfEdge`] with flexible selectors
///
/// This trait provides a more flexible interface for updating half-edges, allowing
/// objects to be selected using the `Selector` trait.
pub trait UpdateHalfEdgeWithSelector {
    /// Update curves selected by the given selector
    #[must_use]
    fn update_curves<T>(
        &self,
        selector: impl Selector<Curve>,
        update: impl Fn(&Handle<Curve>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Curve>>;

    /// Update start vertices selected by the given selector
    #[must_use]
    fn update_start_vertices<T>(
        &self,
        selector: impl Selector<Vertex>,
        update: impl Fn(&Handle<Vertex>, &mut Core) -> T,
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

impl UpdateHalfEdgeWithSelector for HalfEdge {
    fn update_curves<T>(
        &self,
        selector: impl Selector<Curve>,
        update: impl Fn(&Handle<Curve>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Curve>>,
    {
        // For HalfEdge, there's only one curve, so the selector selects from a single-item set
        use crate::topology::ObjectSet;
        let curve_set = ObjectSet::new([self.curve().clone()]);
        let selected_handles: Vec<_> = selector.select(&curve_set).collect();

        if let Some(curve_handle) = selected_handles.first() {
            let updated_curve = update(curve_handle, core);
            HalfEdge::new(
                updated_curve.insert(core).derive_from(self.curve(), core),
                self.start_vertex().clone(),
            )
        } else {
            self.clone()
        }
    }

    fn update_start_vertices<T>(
        &self,
        selector: impl Selector<Vertex>,
        update: impl Fn(&Handle<Vertex>, &mut Core) -> T,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Vertex>>,
    {
        // For HalfEdge, there's only one start vertex, so the selector selects from a single-item set
        use crate::topology::ObjectSet;
        let vertex_set = ObjectSet::new([self.start_vertex().clone()]);
        let selected_handles: Vec<_> = selector.select(&vertex_set).collect();

        if let Some(vertex_handle) = selected_handles.first() {
            let updated_vertex = update(vertex_handle, core);
            HalfEdge::new(
                self.curve().clone(),
                updated_vertex
                    .insert(core)
                    .derive_from(self.start_vertex(), core),
            )
        } else {
            self.clone()
        }
    }
}
