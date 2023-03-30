use fj_interop::ext::ArrayExt;
use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::curve::Curve,
    objects::{GlobalEdge, HalfEdge, Objects, Surface, Vertex},
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

    /// Create an arc
    ///
    /// # Panics
    ///
    /// Panics if the given angle is not within the range (-2pi, 2pi) radians.
    fn arc(
        start: impl Into<Point<2>>,
        end: impl Into<Point<2>>,
        angle_rad: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> HalfEdge {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let curve =
            Curve::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        HalfEdge::unjoined(curve, boundary, objects)
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

    /// Create a line segment
    fn line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        boundary: Option<[Point<1>; 2]>,
        objects: &mut Service<Objects>,
    ) -> HalfEdge {
        let boundary =
            boundary.unwrap_or_else(|| [[0.], [1.]].map(Point::from));
        let curve = Curve::line_from_points_with_coords(
            boundary.zip_ext(points_surface),
        );

        HalfEdge::unjoined(curve, boundary, objects)
    }

    /// Create a line segment from global points
    fn line_segment_from_global_points(
        points_global: [impl Into<Point<3>>; 2],
        surface: &Surface,
        boundary: Option<[Point<1>; 2]>,
        objects: &mut Service<Objects>,
    ) -> HalfEdge {
        let points_surface = points_global
            .map(|point| surface.geometry().project_global_point(point));
        HalfEdge::line_segment(points_surface, boundary, objects)
    }
}

impl BuildHalfEdge for HalfEdge {}
