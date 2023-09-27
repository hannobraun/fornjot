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
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn update_edge(
        &self,
        handle: &Handle<Edge>,
        update: impl FnOnce(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self;

    /// Replace the provided edge
    ///
    /// This is a more general version of [`UpdateCycle::update_edge`] which can
    /// replace a single edge with multiple others.
    ///
    /// # Panics
    ///
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn replace_edge<const N: usize>(
        &self,
        handle: &Handle<Edge>,
        replace: impl FnOnce(&Handle<Edge>) -> [Handle<Edge>; N],
    ) -> Self;
}

impl UpdateCycle for Cycle {
    fn add_edges(&self, edges: impl IntoIterator<Item = Handle<Edge>>) -> Self {
        let edges = self.edges().iter().cloned().chain(edges);
        Cycle::new(edges)
    }

    fn update_edge(
        &self,
        handle: &Handle<Edge>,
        update: impl FnOnce(&Handle<Edge>) -> Handle<Edge>,
    ) -> Self {
        let edges = self.edges().update(handle, update);
        Cycle::new(edges)
    }

    fn replace_edge<const N: usize>(
        &self,
        handle: &Handle<Edge>,
        replace: impl FnOnce(&Handle<Edge>) -> [Handle<Edge>; N],
    ) -> Self {
        let edges = self.edges().replace(handle, replace);
        Cycle::new(edges)
    }
}
