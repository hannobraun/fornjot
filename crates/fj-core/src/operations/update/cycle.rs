use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, selector::Selector},
    storage::Handle,
    topology::{Cycle, HalfEdge},
};

/// Update a [`Cycle`]
pub trait UpdateCycle {
    /// Add edges to the cycle
    #[must_use]
    fn add_half_edges<T>(
        &self,
        half_edges: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>;

    /// Update an edge of the cycle
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_half_edge<T, R>(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
        R: IntoIterator<Item = T>;
}

/// Update a [`Cycle`] with flexible selectors
///
/// This trait provides a more flexible interface for updating cycles, allowing
/// objects to be selected using the `Selector` trait.
pub trait UpdateCycleWithSelector {
    /// Add edges to the cycle
    #[must_use]
    fn add_half_edges<T>(
        &self,
        half_edges: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>;

    /// Update half-edges selected by the given selector
    ///
    /// # Panics
    ///
    /// Panics, if any selected object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_half_edges<T, R>(
        &self,
        selector: impl Selector<HalfEdge>,
        update: impl Fn(&Handle<HalfEdge>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
        R: IntoIterator<Item = T>;
}

impl UpdateCycle for Cycle {
    fn add_half_edges<T>(
        &self,
        half_edges: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
    {
        let half_edges = half_edges
            .into_iter()
            .map(|half_edge| half_edge.insert(core));
        let half_edges = self.half_edges().iter().cloned().chain(half_edges);
        Cycle::new(half_edges)
    }

    fn update_half_edge<T, R>(
        &self,
        handle: &Handle<HalfEdge>,
        update: impl FnOnce(&Handle<HalfEdge>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
        R: IntoIterator<Item = T>,
    {
        let edges = self
            .half_edges()
            .replace(
                handle,
                update(handle, core).into_iter().map(|object| {
                    object.insert(core).derive_from(handle, core)
                }),
            )
            .expect("Half-edge not found");
        Cycle::new(edges)
    }
}

impl UpdateCycleWithSelector for Cycle {
    fn add_half_edges<T>(
        &self,
        half_edges: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
    {
        let half_edges = half_edges
            .into_iter()
            .map(|half_edge| half_edge.insert(core));
        let half_edges = self.half_edges().iter().cloned().chain(half_edges);
        Cycle::new(half_edges)
    }

    fn update_half_edges<T, R>(
        &self,
        selector: impl Selector<HalfEdge>,
        update: impl Fn(&Handle<HalfEdge>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<HalfEdge>>,
        R: IntoIterator<Item = T>,
    {
        let selected_handles: Vec<_> =
            selector.select(self.half_edges()).collect();

        let mut result = self.clone();
        for handle in selected_handles {
            result = result.update_half_edge(handle, &update, core);
        }
        result
    }
}
