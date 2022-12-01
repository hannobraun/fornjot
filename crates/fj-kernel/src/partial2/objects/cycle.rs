use crate::{objects::HalfEdge, partial2::Partial};

/// A partial [`Cycle`]
///
/// [`Cycle`]: crate::objects::Cycle
pub struct PartialCycle {
    /// The half-edges that make up the cycle
    pub half_edges: Vec<Partial<HalfEdge>>,
}
