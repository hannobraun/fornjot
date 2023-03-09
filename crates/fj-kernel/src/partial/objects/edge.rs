use fj_math::Point;

use crate::{
    geometry::curve::Curve,
    objects::{GlobalEdge, HalfEdge, Objects, Vertex},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
    storage::Handle,
};

/// A partial [`HalfEdge`]
#[derive(Clone, Debug)]
pub struct PartialHalfEdge {
    /// The curve that the half-edge is defined in
    pub curve: Curve,

    /// The boundary of the half-edge on the curve
    pub boundary: [Point<1>; 2],

    /// The surface vertex where the half-edge starts
    pub start_vertex: Handle<Vertex>,

    /// The global form of the half-edge
    pub global_form: Handle<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Compute the surface position where the half-edge starts
    pub fn start_position(&self) -> Option<Point<2>> {
        // Computing the surface position from the curve position is fine.
        // `HalfEdge` "owns" its start position. There is no competing code that
        // could compute the surface position from slightly different data.

        let [start, _] = self.boundary;
        Some(self.curve.point_from_path_coords(start))
    }
}

impl PartialObject for PartialHalfEdge {
    type Full = HalfEdge;

    fn new(_: &mut Service<Objects>) -> Self {
        // This method is no longer used, and since `PartialHalfEdge` will be
        // replaced with `HalfEdge`, it will soon be removed.
        unreachable!()
    }

    fn from_full(half_edge: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            curve: half_edge.curve(),
            boundary: half_edge.boundary(),
            start_vertex: half_edge.start_vertex().clone(),
            global_form: half_edge.global_form().clone(),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        HalfEdge::new(
            self.curve,
            self.boundary,
            self.start_vertex,
            self.global_form,
        )
    }
}
