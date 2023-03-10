use crate::{
    objects::{Cycle, HalfEdge, Objects},
    partial::{FullOrPartial, FullToPartialCache, PartialObject},
    services::Service,
};

/// A partial [`Cycle`]
#[derive(Clone, Debug)]
pub struct PartialCycle {
    /// The half-edges that make up the cycle
    pub half_edges: Vec<FullOrPartial<HalfEdge>>,
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
            half_edges: cycle
                .half_edges()
                .cloned()
                .map(|half_edge| half_edge.into())
                .collect(),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let half_edges =
            self.half_edges
                .into_iter()
                .map(|half_edge| match half_edge {
                    FullOrPartial::Full(half_edge) => half_edge,
                    FullOrPartial::Partial(half_edge) => {
                        half_edge.build(objects)
                    }
                });

        Cycle::new(half_edges)
    }
}
