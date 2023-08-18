use crate::{
    objects::{Cycle, Edge},
    storage::Handle,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add half-edges to the cycle
    #[must_use]
    fn add_half_edges(
        &self,
        half_edges: impl IntoIterator<Item = Handle<Edge>>,
    ) -> Self;

    /// Replace the provided half-edge
    ///
    /// # Panics
    ///
    /// Panics, unless this operation replaces exactly one half-edge.
    #[must_use]
    fn replace_half_edge(
        &self,
        original: &Handle<Edge>,
        replacement: Handle<Edge>,
    ) -> Self;

    /// Update the half-edge at the given index
    ///
    /// # Panics
    ///
    /// Panics, unless this operation updates exactly one half-edge.
    #[must_use]
    fn update_nth_half_edge(
        &self,
        index: usize,
        f: impl FnMut(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self;
}

impl UpdateCycle for Cycle {
    fn add_half_edges(
        &self,
        half_edges: impl IntoIterator<Item = Handle<Edge>>,
    ) -> Self {
        let half_edges = self.half_edges().cloned().chain(half_edges);
        Cycle::new(half_edges)
    }

    fn replace_half_edge(
        &self,
        original: &Handle<Edge>,
        replacement: Handle<Edge>,
    ) -> Self {
        let mut num_replacements = 0;

        let half_edges = self.half_edges().map(|half_edge| {
            if half_edge.id() == original.id() {
                num_replacements += 1;
                replacement.clone()
            } else {
                half_edge.clone()
            }
        });

        let cycle = Cycle::new(half_edges);

        assert_eq!(
            num_replacements, 1,
            "Expected operation to replace exactly one half-edge"
        );

        cycle
    }

    fn update_nth_half_edge(
        &self,
        index: usize,
        mut f: impl FnMut(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self {
        let mut num_replacements = 0;

        let half_edges = self.half_edges().enumerate().map(|(i, half_edge)| {
            if i == index {
                num_replacements += 1;
                f(half_edge)
            } else {
                half_edge.clone()
            }
        });

        let cycle = Cycle::new(half_edges);

        assert_eq!(
            num_replacements, 1,
            "Expected operation to replace exactly one half-edge"
        );

        cycle
    }
}
