use crate::kernel::topology::edges::{Cycle, Edge};

use super::{
    handle::{Handle, Storage},
    CyclesInner,
};

/// The cycles of a shape
pub struct Cycles<'r> {
    pub(super) cycles: &'r mut CyclesInner,
}

impl Cycles<'_> {
    /// Create a cycle
    ///
    /// # Implementation note
    ///
    /// This method should at some point validate the cycle:
    /// - That it refers to valid edges that are part of `Shape`.
    /// - That those edges form a cycle.
    /// - That the cycle is not self-overlapping.
    /// - That there exists no duplicate cycle, with the same edges.
    pub fn create(
        &mut self,
        edges: impl IntoIterator<Item = Handle<Edge>>,
    ) -> Handle<Cycle> {
        let cycle = Cycle {
            edges: edges.into_iter().collect(),
        };

        self.cycles.push(cycle.clone());

        let storage = Storage::new(cycle);
        storage.handle()
    }

    /// Access an iterator over all cycles
    pub fn all(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.cycles.iter().cloned()
    }
}
