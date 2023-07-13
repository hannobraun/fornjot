use fj_interop::ext::ArrayExt;
use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::SurfacePath,
    objects::{GlobalEdge, HalfEdge, Vertex},
    operations::Insert,
    services::Services,
};

/// Build a [`HalfEdge`]
pub trait BuildHalfEdge {
    /// Create a half-edge that is not joined to another
    fn unjoined(
        curve: SurfacePath,
        boundary: [Point<1>; 2],
        services: &mut Services,
    ) -> HalfEdge {
        let start_vertex = Vertex::new().insert(services);
        let global_form = GlobalEdge::new().insert(services);

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
        services: &mut Services,
    ) -> HalfEdge {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let curve =
            SurfacePath::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        HalfEdge::unjoined(curve, boundary, services)
    }

    /// Create a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> HalfEdge {
        let curve = SurfacePath::circle_from_center_and_radius(center, radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        HalfEdge::unjoined(curve, boundary, services)
    }

    /// Create a line segment
    fn line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        boundary: Option<[Point<1>; 2]>,
        services: &mut Services,
    ) -> HalfEdge {
        let boundary =
            boundary.unwrap_or_else(|| [[0.], [1.]].map(Point::from));
        let curve = SurfacePath::line_from_points_with_coords(
            boundary.zip_ext(points_surface),
        );

        HalfEdge::unjoined(curve, boundary, services)
    }
}

impl BuildHalfEdge for HalfEdge {}
