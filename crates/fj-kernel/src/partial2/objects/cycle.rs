use crate::{
    objects::{Cycle, HalfEdge},
    partial2::{Partial, PartialObject},
};

/// A partial [`Cycle`]
pub struct PartialCycle {
    /// The half-edges that make up the cycle
    pub half_edges: Vec<Partial<HalfEdge>>,
}

impl PartialObject for PartialCycle {
    type Full = Cycle;
}
