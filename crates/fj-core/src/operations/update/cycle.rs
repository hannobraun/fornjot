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
        original: &Handle<Edge>,
        replacement: Handle<Edge>,
    ) -> Self;

    /// Update the edge at the given index
    ///
    /// # Panics
    ///
    /// Panics, unless this operation updates exactly one edge.
    #[must_use]
    fn update_nth_edge(
        &self,
        index: usize,
        f: impl FnMut(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self;
}

impl UpdateCycle for Cycle {
    fn add_edges(&self, edges: impl IntoIterator<Item = Handle<Edge>>) -> Self {
        let edges = self.edges().iter().cloned().chain(edges);
        Cycle::new(edges)
    }

    fn update_edge(
        &self,
        original: &Handle<Edge>,
        replacement: Handle<Edge>,
    ) -> Self {
        let mut num_replacements = 0;

        let edges = self.edges().iter().map(|edge| {
            if edge.id() == original.id() {
                num_replacements += 1;
                replacement.clone()
            } else {
                edge.clone()
            }
        });

        let cycle = Cycle::new(edges);

        assert_eq!(
            num_replacements, 1,
            "Expected operation to replace exactly one edge"
        );

        cycle
    }

    fn update_nth_edge(
        &self,
        index: usize,
        mut f: impl FnMut(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self {
        let mut num_replacements = 0;

        let edges = self.edges().iter().enumerate().map(|(i, edge)| {
            if i == index {
                num_replacements += 1;
                f(edge)
            } else {
                edge.clone()
            }
        });

        let cycle = Cycle::new(edges);

        assert_eq!(
            num_replacements, 1,
            "Expected operation to replace exactly one edge"
        );

        cycle
    }
}
