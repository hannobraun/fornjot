use fj_interop::ext::ArrayExt;
use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Curve, Edge, Vertex},
    operations::Insert,
    services::Services,
};

/// Build an [`Edge`]
pub trait BuildEdge {
    /// Create an edge that is not joined to another
    fn unjoined(
        path: SurfacePath,
        boundary: impl Into<CurveBoundary<Point<1>>>,
        services: &mut Services,
    ) -> Edge {
        let curve = Curve::new().insert(services);
        let start_vertex = Vertex::new().insert(services);

        Edge::new(path, boundary, curve, start_vertex)
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
    ) -> Edge {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let path =
            SurfacePath::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        Edge::unjoined(path, boundary, services)
    }

    /// Create a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> Edge {
        let path = SurfacePath::circle_from_center_and_radius(center, radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        Edge::unjoined(path, boundary, services)
    }

    /// Create a line segment
    fn line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        boundary: Option<[Point<1>; 2]>,
        services: &mut Services,
    ) -> Edge {
        let boundary =
            boundary.unwrap_or_else(|| [[0.], [1.]].map(Point::from));
        let path = SurfacePath::line_from_points_with_coords(
            boundary.zip_ext(points_surface),
        );

        Edge::unjoined(path, boundary, services)
    }
}

impl BuildEdge for Edge {}
