use fj_math::{Point, Scalar};

use crate::{
    geometry::curve::Curve,
    objects::{GlobalEdge, HalfEdge, Objects, Vertex},
    operations::Insert,
    services::Service,
};

/// Build a [`HalfEdge`]
pub trait BuildHalfEdge {
    /// Create a half-edge that is not joined to another
    fn unjoined(
        curve: Curve,
        boundary: [Point<1>; 2],
        objects: &mut Service<Objects>,
    ) -> HalfEdge {
        let start_vertex = Vertex::new().insert(objects);
        let global_form = GlobalEdge::new().insert(objects);

        HalfEdge::new(curve, boundary, start_vertex, global_form)
    }

    /// Create a circle
    fn circle(
        radius: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> HalfEdge {
        let curve = Curve::circle_from_radius(radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        HalfEdge::unjoined(curve, boundary, objects)
    }
}

impl BuildHalfEdge for HalfEdge {}
