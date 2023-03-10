use fj_interop::ext::ArrayExt;
use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::curve::Curve,
    insert::Insert,
    objects::{GlobalEdge, HalfEdge, Objects, Vertex},
    services::Service,
    storage::Handle,
};

/// Builder API for [`HalfEdge`]
pub struct HalfEdgeBuilder {}

impl HalfEdgeBuilder {
    /// Create a circle
    pub fn circle(
        radius: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Handle<HalfEdge> {
        let curve = Curve::circle_from_radius(radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        Self::make_half_edge(curve, boundary, None, None, objects)
    }

    /// Create an arc
    ///
    /// # Panics
    ///
    /// Panics if the given angle is not within the range (-2pi, 2pi) radians.
    pub fn make_arc(
        start: impl Into<Point<2>>,
        end: impl Into<Point<2>>,
        angle_rad: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Handle<HalfEdge> {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let curve =
            Curve::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        Self::make_half_edge(curve, boundary, None, None, objects)
    }

    /// Create a line segment
    pub fn make_line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        boundary: Option<[Point<1>; 2]>,
        start_vertex: Option<Handle<Vertex>>,
        global_form: Option<Handle<GlobalEdge>>,
        objects: &mut Service<Objects>,
    ) -> Handle<HalfEdge> {
        let boundary =
            boundary.unwrap_or_else(|| [[0.], [1.]].map(Point::from));
        let curve = Curve::line_from_points_with_coords(
            boundary.zip_ext(points_surface),
        );

        Self::make_half_edge(
            curve,
            boundary,
            start_vertex,
            global_form,
            objects,
        )
    }

    /// Create a half-edge
    pub fn make_half_edge(
        curve: Curve,
        boundary: [Point<1>; 2],
        start_vertex: Option<Handle<Vertex>>,
        global_form: Option<Handle<GlobalEdge>>,
        objects: &mut Service<Objects>,
    ) -> Handle<HalfEdge> {
        HalfEdge::new(
            curve,
            boundary,
            start_vertex.unwrap_or_else(|| Vertex::new().insert(objects)),
            global_form.unwrap_or_else(|| GlobalEdge::new().insert(objects)),
        )
        .insert(objects)
    }
}
