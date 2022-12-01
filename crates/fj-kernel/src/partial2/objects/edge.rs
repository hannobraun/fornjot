use crate::{
    objects::{GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Vertex},
    partial2::{Partial, PartialObject},
};

/// A partial [`HalfEdge`]
pub struct PartialHalfEdge {
    /// The vertices that bound the half-edge on the curve
    pub vertices: [Partial<Vertex>; 2],

    /// The global form of the half-edge
    pub global_form: Partial<GlobalEdge>,
}

impl PartialObject for PartialHalfEdge {
    type Full = HalfEdge;
}

/// A partial [`GlobalEdge`]
pub struct PartialGlobalEdge {
    /// The curve that defines the edge's geometry
    pub curve: Partial<GlobalCurve>,

    /// The vertices that bound the edge on the curve
    pub vertices: [Partial<GlobalVertex>; 2],
}

impl PartialObject for PartialGlobalEdge {
    type Full = GlobalEdge;
}
