use crate::{
    objects::{Cycle, HalfEdge, Objects},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Cycle`]
#[derive(Clone, Debug)]
pub struct PartialCycle {
    /// The half-edges that make up the cycle
    pub half_edges: Vec<Partial<HalfEdge>>,
}

impl PartialObject for PartialCycle {
    type Full = Cycle;

    fn new(_: &mut Service<Objects>) -> Self {
        Self {
            half_edges: Vec::new(),
        }
    }

    fn from_full(cycle: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            half_edges: cycle
                .half_edges()
                .cloned()
                .map(|half_edge| Partial::from_full(half_edge, cache))
                .collect(),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|half_edge| half_edge.build(objects));

        Cycle::new(half_edges)
    }
}
