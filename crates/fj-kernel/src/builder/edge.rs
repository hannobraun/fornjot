use crate::{
    objects::{Curve, Vertex},
    partial::PartialGlobalEdge,
    storage::Handle,
};

/// Builder API for [`PartialGlobalEdge`]
#[allow(clippy::wrong_self_convention)]
pub trait GlobalEdgeBuilder {
    /// Update partial global edge from the given curve and vertices
    fn from_curve_and_vertices(
        self,
        curve: &Curve,
        vertices: &[Handle<Vertex>; 2],
    ) -> Self;
}

impl GlobalEdgeBuilder for PartialGlobalEdge {
    fn from_curve_and_vertices(
        self,
        curve: &Curve,
        vertices: &[Handle<Vertex>; 2],
    ) -> Self {
        self.with_curve(Some(curve.global_form().clone()))
            .with_vertices(Some(
                vertices.clone().map(|vertex| vertex.global_form().clone()),
            ))
    }
}
