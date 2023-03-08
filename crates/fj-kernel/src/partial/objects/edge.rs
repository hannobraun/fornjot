use fj_math::Point;

use crate::{
    geometry::curve::Curve,
    insert::Insert,
    objects::{GlobalEdge, HalfEdge, Objects, Vertex},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
    storage::Handle,
};

/// A partial [`HalfEdge`]
#[derive(Clone, Debug)]
pub struct PartialHalfEdge {
    /// The curve that the half-edge is defined in
    pub curve: Option<Curve>,

    /// The boundary of the half-edge on the curve
    pub boundary: [Option<Point<1>>; 2],

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
        start.and_then(|start| {
            let curve = self.curve?;
            Some(curve.point_from_path_coords(start))
        })
    }
}

impl PartialObject for PartialHalfEdge {
    type Full = HalfEdge;

    fn new(objects: &mut Service<Objects>) -> Self {
        Self {
            curve: None,
            boundary: [None; 2],
            start_vertex: Vertex::new().insert(objects),
            global_form: GlobalEdge::new().insert(objects),
        }
    }

    fn from_full(half_edge: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            curve: Some(half_edge.curve()),
            boundary: half_edge.boundary().map(Some),
            start_vertex: half_edge.start_vertex().clone(),
            global_form: half_edge.global_form().clone(),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let curve = self.curve.expect("Need path to build curve");
        let boundary = self.boundary.map(|point| {
            point.expect("Can't build `HalfEdge` without boundary positions")
        });

        HalfEdge::new(curve, boundary, self.start_vertex, self.global_form)
    }
}

/// A possibly undefined curve
#[derive(Clone, Copy, Debug)]
pub enum MaybeCurve {
    /// The curve is fully defined
    Defined(Curve),
}

impl From<Curve> for MaybeCurve {
    fn from(path: Curve) -> Self {
        Self::Defined(path)
    }
}
