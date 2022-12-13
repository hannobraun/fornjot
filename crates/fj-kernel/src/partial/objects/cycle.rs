use crate::{
    objects::{Cycle, HalfEdge, Objects, Surface},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Cycle`]
#[derive(Clone, Debug)]
pub struct PartialCycle {
    /// The half-edges that make up the cycle
    pub half_edges: Vec<Partial<HalfEdge>>,
}

impl PartialCycle {
    /// Construct an instance of `PartialCycle`
    pub fn new() -> Self {
        Self {
            half_edges: Vec::new(),
        }
    }

    /// Access the surface of the [`Cycle`]
    pub fn surface(&self) -> Option<Partial<Surface>> {
        self.half_edges
            .first()
            .map(|half_edge| half_edge.read().curve().read().surface.clone())
    }
}

impl PartialObject for PartialCycle {
    type Full = Cycle;

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

impl Default for PartialCycle {
    fn default() -> Self {
        Self::new()
    }
}
