use crate::{
    objects::{Cycle, Edge},
    storage::Handle,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add edges to the cycle
    #[must_use]
    fn add_edges(&self, edges: impl IntoIterator<Item = Handle<Edge>>) -> Self;

    /// Update the provided edge
    ///
    /// # Panics
    ///
    /// Panics, if the provided edge is not part of the cycle.
    #[must_use]
    fn update_edge(
        &self,
        edge: &Handle<Edge>,
        update: impl FnOnce(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self;
}

impl UpdateCycle for Cycle {
    fn add_edges(&self, edges: impl IntoIterator<Item = Handle<Edge>>) -> Self {
        let edges = self.edges().iter().cloned().chain(edges);
        Cycle::new(edges)
    }

    fn update_edge(
        &self,
        edge: &Handle<Edge>,
        update: impl FnOnce(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self {
        let edges = self.edges().update(edge, update);
        Cycle::new(edges)
    }
}
