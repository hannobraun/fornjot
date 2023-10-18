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
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn update_edge(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>) -> Handle<HalfEdge>,
    ) -> Self;

    /// Replace an edge of the cycle
    ///
    /// This is a more general version of [`UpdateCycle::update_edge`] which can
    /// replace a single edge with multiple others.
    ///
    /// # Panics
    ///
    /// Uses [`Handles::replace`] internally, and panics for the same reasons.
    ///
    /// [`Handles::replace`]: crate::objects::Handles::replace
    #[must_use]
    fn replace_edge<const N: usize>(
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

    fn update_edge(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>) -> Handle<HalfEdge>,
    ) -> Self {
        let edges = self.half_edges().update(handle, update);
        Cycle::new(edges)
    }

    fn replace_edge<const N: usize>(
        &self,
        handle: &Handle<HalfEdge>,
        replace: impl FnOnce(&Handle<HalfEdge>) -> [Handle<HalfEdge>; N],
    ) -> Self {
        let edges = self.half_edges().replace(handle, replace);
        Cycle::new(edges)
    }
}
