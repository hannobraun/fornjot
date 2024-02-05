use fj_interop::ext::ArrayExt;
use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Curve, HalfEdge, Vertex},
    operations::insert::Insert,
    services::Services,
    storage::Handle,
    Instance,
};

/// Build a [`HalfEdge`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildHalfEdge {
    /// Create a half-edge that is not joined to a sibling
    fn unjoined(
        path: SurfacePath,
        boundary: impl Into<CurveBoundary<Point<1>>>,
        services: &mut Services,
    ) -> HalfEdge {
        let curve = Curve::new().insert(services);
        let start_vertex = Vertex::new().insert(services);

        HalfEdge::new(path, boundary, curve, start_vertex)
    }

    /// Create a half-edge from its sibling
    fn from_sibling(
        sibling: &HalfEdge,
        start_vertex: Handle<Vertex>,
    ) -> HalfEdge {
        HalfEdge::new(
            sibling.path(),
            sibling.boundary().reverse(),
            sibling.curve().clone(),
            start_vertex,
        )
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
        core: &mut Instance,
    ) -> HalfEdge {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let path =
            SurfacePath::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        HalfEdge::unjoined(path, boundary, &mut core.services)
    }

    /// Create a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> HalfEdge {
        let path = SurfacePath::circle_from_center_and_radius(center, radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        HalfEdge::unjoined(path, boundary, services)
    }

    /// Create a line segment
    fn line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        boundary: Option<[Point<1>; 2]>,
        services: &mut Services,
    ) -> HalfEdge {
        let boundary =
            boundary.unwrap_or_else(|| [[0.], [1.]].map(Point::from));
        let path = SurfacePath::line_from_points_with_coords(
            boundary.zip_ext(points_surface),
        );

        HalfEdge::unjoined(path, boundary, services)
    }
}

impl BuildHalfEdge for HalfEdge {}
