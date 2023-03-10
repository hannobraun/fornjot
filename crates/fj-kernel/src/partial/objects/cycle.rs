use crate::{
    objects::{Cycle, HalfEdge, Objects},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
    storage::Handle,
};

/// A partial [`Cycle`]
#[derive(Clone, Debug)]
pub struct PartialCycle {
    /// The half-edges that make up the cycle
    pub half_edges: Vec<Handle<HalfEdge>>,
}

impl PartialObject for PartialCycle {
    type Full = Cycle;

    fn new(_: &mut Service<Objects>) -> Self {
        Self {
            half_edges: Vec::new(),
        }
    }

    fn from_full(cycle: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            half_edges: cycle.half_edges().cloned().collect(),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let half_edges = self.half_edges;

        Cycle::new(half_edges)
    }
}
