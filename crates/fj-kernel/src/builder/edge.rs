use fj_math::Point;

use crate::{
    geometry::curve::Curve,
    objects::{GlobalEdge, HalfEdge, Objects, Vertex},
    operations::Insert,
    services::Service,
    storage::Handle,
};

/// Builder API for [`HalfEdge`]
pub struct HalfEdgeBuilder {
    curve: Curve,
    boundary: [Point<1>; 2],
    start_vertex: Option<Handle<Vertex>>,
    global_form: Option<Handle<GlobalEdge>>,
}

impl HalfEdgeBuilder {
    /// Build the half-edge
    pub fn build(self, objects: &mut Service<Objects>) -> HalfEdge {
        HalfEdge::new(
            self.curve,
            self.boundary,
            self.start_vertex
                .unwrap_or_else(|| Vertex::new().insert(objects)),
            self.global_form
                .unwrap_or_else(|| GlobalEdge::new().insert(objects)),
        )
    }
}
