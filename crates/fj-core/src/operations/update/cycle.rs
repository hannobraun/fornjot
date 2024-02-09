use crate::{
    objects::{Cycle, HalfEdge},
    storage::Handle,
    Instance,
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add edges to the cycle
    #[must_use]
    fn add_half_edges(
        &self,
        half_edges: impl IntoIterator<Item = Handle<HalfEdge>>,
    ) -> Self;

    /// Update an edge of the cycle
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_half_edge<const N: usize>(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(
            &Handle<HalfEdge>,
            &mut Instance,
        ) -> [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> Self;
}

impl UpdateCycle for Cycle {
    fn add_half_edges(
        &self,
        half_edges: impl IntoIterator<Item = Handle<HalfEdge>>,
    ) -> Self {
        let half_edges = self.half_edges().iter().cloned().chain(half_edges);
        Cycle::new(half_edges)
    }

    fn update_half_edge<const N: usize>(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(
            &Handle<HalfEdge>,
            &mut Instance,
        ) -> [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> Self {
        let edges = self
            .half_edges()
            .replace(handle, update(handle, core))
            .expect("Half-edge not found");
        Cycle::new(edges)
    }
}
