use crate::{
    objects::{Cycle, HalfEdge},
    operations::insert::Insert,
    storage::Handle,
    Core,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add edges to the cycle
    #[must_use]
    fn add_half_edges<T>(
        &self,
        half_edges: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>;

    /// Update an edge of the cycle
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_half_edge<T, const N: usize>(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>, &mut Core) -> [T; N],
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>;
}

impl UpdateCycle for Cycle {
    fn add_half_edges<T>(
        &self,
        half_edges: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
    {
        let half_edges = half_edges
            .into_iter()
            .map(|half_edge| half_edge.insert(core));
        let half_edges = self.half_edges().iter().cloned().chain(half_edges);
        Cycle::new(half_edges)
    }

    fn update_half_edge<T, const N: usize>(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>, &mut Core) -> [T; N],
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
    {
        let edges = self
            .half_edges()
            .replace(
                handle,
                update(handle, core).map(|object| object.insert(core)),
            )
            .expect("Half-edge not found");
        Cycle::new(edges)
    }
}
