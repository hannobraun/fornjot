use crate::{
    objects::{Cycle, HalfEdge},
    storage::Handle,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Update a half-edge of the cycle
    fn update_half_edge(
        &self,
        index: usize,
        f: impl FnMut(&Handle<HalfEdge>) -> Handle<HalfEdge>,
    ) -> Cycle;
}

impl UpdateCycle for Cycle {
    fn update_half_edge(
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
