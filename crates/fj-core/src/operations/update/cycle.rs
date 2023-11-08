use crate::{
    objects::{Cycle, HalfEdge},
    storage::Handle,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add edges to the cycle
    #[must_use]
    fn add_half_edges(
        &self,
        edges: impl IntoIterator<Item = Handle<HalfEdge>>,
    ) -> Self;

    /// Update an edge of the cycle
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_half_edge(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>) -> Handle<HalfEdge>,
    ) -> Self;

    /// Replace an edge of the cycle
    ///
    /// This is a more general version of [`UpdateCycle::update_half_edge`]
    /// which can replace a single edge with multiple others.
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn replace_half_edge<const N: usize>(
        &self,
        handle: &Handle<HalfEdge>,
        replace: impl FnOnce(&Handle<HalfEdge>) -> [Handle<HalfEdge>; N],
    ) -> Self;
}

impl UpdateCycle for Cycle {
    fn add_half_edges(
        &self,
        edges: impl IntoIterator<Item = Handle<HalfEdge>>,
    ) -> Self {
        let edges = self.half_edges().iter().cloned().chain(edges);
        Cycle::new(edges)
    }

    fn update_half_edge(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>) -> Handle<HalfEdge>,
    ) -> Self {
        let edges = self
            .half_edges()
            .replace(handle, [update(handle)])
            .expect("Half-edge not found");
        Cycle::new(edges)
    }

    fn replace_half_edge<const N: usize>(
        &self,
        handle: &Handle<HalfEdge>,
        replace: impl FnOnce(&Handle<HalfEdge>) -> [Handle<HalfEdge>; N],
    ) -> Self {
        let edges = self
            .half_edges()
            .replace(handle, replace(handle))
            .expect("Half-edge not found");
        Cycle::new(edges)
    }
}
