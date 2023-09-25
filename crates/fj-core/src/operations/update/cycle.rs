use crate::{
    objects::{Cycle, Edge},
    storage::Handle,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add edges to the cycle
    #[must_use]
    fn add_edges(&self, edges: impl IntoIterator<Item = Handle<Edge>>) -> Self;

    /// Replace the provided edge
    ///
    /// # Panics
    ///
    /// Panics, unless this operation replaces exactly one edge.
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
        let mut updated = Some(update(edge));

        let edges = self.edges().iter().map(|e| {
            if e.id() == edge.id() {
                updated
                    .take()
                    .expect("Cycle should not contain same edge twice")
            } else {
                e.clone()
            }
        });

        Cycle::new(edges)
    }
}
