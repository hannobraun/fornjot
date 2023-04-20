use crate::{
    objects::{Cycle, HalfEdge},
    storage::Handle,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add half-edges to the cycle
    fn add_half_edges(
        &self,
        half_edges: impl IntoIterator<Item = Handle<HalfEdge>>,
    ) -> Cycle;

    /// Update a half-edge of the cycle
    fn replace_nth_half_edge(
        &self,
        index: usize,
        f: impl FnMut(&Handle<HalfEdge>) -> Handle<HalfEdge>,
    ) -> Cycle;
}

impl UpdateCycle for Cycle {
    fn add_half_edges(
        &self,
        half_edges: impl IntoIterator<Item = Handle<HalfEdge>>,
    ) -> Cycle {
        let half_edges = self.half_edges().cloned().chain(half_edges);
        Cycle::new(half_edges)
    }

    fn replace_nth_half_edge(
        &self,
        index: usize,
        mut f: impl FnMut(&Handle<HalfEdge>) -> Handle<HalfEdge>,
    ) -> Cycle {
        let half_edges = self.half_edges().enumerate().map(|(i, cycle)| {
            if i == index {
                f(cycle)
            } else {
                cycle.clone()
            }
        });

        Cycle::new(half_edges)
    }
}
