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
pub struct HalfEdgeBuilder {
    curve: Curve,
    boundary: [Point<1>; 2],
}

impl HalfEdgeBuilder {
    /// Create an instance of `HalfEdgeBuilder`
    pub fn new(curve: Curve, boundary: [Point<1>; 2]) -> Self {
        Self { curve, boundary }
    }

    /// Create an arc
    ///
    /// # Panics
    ///
    /// Panics if the given angle is not within the range (-2pi, 2pi) radians.
    pub fn arc(
        start: impl Into<Point<2>>,
        end: impl Into<Point<2>>,
        angle_rad: impl Into<Scalar>,
    ) -> Self {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let curve =
            Curve::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        Self::new(curve, boundary)
    }

    /// Create a circle
    pub fn circle(radius: impl Into<Scalar>) -> Self {
        let curve = Curve::circle_from_radius(radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        Self::new(curve, boundary)
    }

    /// Create a line segment
    pub fn line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        boundary: Option<[Point<1>; 2]>,
    ) -> Self {
        let boundary =
            boundary.unwrap_or_else(|| [[0.], [1.]].map(Point::from));
        let curve = Curve::line_from_points_with_coords(
            boundary.zip_ext(points_surface),
        );

        Self::new(curve, boundary)
    }

    /// Create a half-edge
    pub fn build(
        self,
        start_vertex: Option<Handle<Vertex>>,
        global_form: Option<Handle<GlobalEdge>>,
        objects: &mut Service<Objects>,
    ) -> Handle<HalfEdge> {
        HalfEdge::new(
            self.curve,
            self.boundary,
            start_vertex.unwrap_or_else(|| Vertex::new().insert(objects)),
            global_form.unwrap_or_else(|| GlobalEdge::new().insert(objects)),
        )
        .insert(objects)
    }
}
