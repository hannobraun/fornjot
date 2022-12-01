use crate::{
    objects::{GlobalCurve, GlobalEdge, GlobalVertex, Vertex},
    partial2::Partial,
};

/// A partial [`HalfEdge`]
///
/// [`HalfEdge`]: crate::objects::HalfEdge
pub struct PartialHalfEdge {
    /// The vertices that bound the half-edge on the curve
    pub vertices: [Partial<Vertex>; 2],

    /// The global form of the half-edge
    pub global_form: Partial<GlobalEdge>,
}

/// A partial [`GlobalEdge`]
///
/// [`GlobalEdge`]: crate::objects::GlobalEdge
pub struct PartialGlobalEdge {
    /// The curve that defines the edge's geometry
    pub curve: Partial<GlobalCurve>,

    /// The vertices that bound the edge on the curve
    pub vertices: [Partial<GlobalVertex>; 2],
}
