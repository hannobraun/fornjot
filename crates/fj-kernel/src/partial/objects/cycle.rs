use crate::{
    builder::CycleBuilder,
    objects::{Cycle, HalfEdge, Objects, Surface},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Cycle`]
#[derive(Clone, Debug, Default)]
pub struct PartialCycle {
    /// The surface that the cycle is defined in
    pub surface: Partial<Surface>,

    /// The half-edges that make up the cycle
    pub half_edges: Vec<Partial<HalfEdge>>,
}

impl PartialObject for PartialCycle {
    type Full = Cycle;

    fn from_full(cycle: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            surface: Partial::from_full(cycle.surface().clone(), cache),
            half_edges: cycle
                .half_edges()
                .cloned()
                .map(|half_edge| Partial::from_full(half_edge, cache))
                .collect(),
        }
    }

    fn build(mut self, objects: &mut Service<Objects>) -> Self::Full {
        self.infer_vertex_positions_if_necessary();

        let surface = self.surface.build(objects);
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|half_edge| half_edge.build(objects));

        Cycle::new(surface, half_edges)
    }
}
