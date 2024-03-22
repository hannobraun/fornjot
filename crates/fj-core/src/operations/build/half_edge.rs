use fj_interop::ext::ArrayExt;
use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::{CurveBoundary, HalfEdgeGeometry, SurfacePath},
    objects::{Curve, HalfEdge, Vertex},
    operations::{geometry::UpdateHalfEdgeGeometry, insert::Insert},
    storage::Handle,
    Core,
};

/// Build a [`HalfEdge`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildHalfEdge {
    /// Create a half-edge that is not joined to a sibling
    fn unjoined(
        boundary: impl Into<CurveBoundary<Point<1>>>,
        core: &mut Core,
    ) -> HalfEdge {
        let curve = Curve::new().insert(core);
        let start_vertex = Vertex::new().insert(core);

        HalfEdge::new(boundary, curve, start_vertex)
    }

    /// Create a half-edge from its sibling
    fn from_sibling(
        sibling: &Handle<HalfEdge>,
        start_vertex: Handle<Vertex>,
        core: &mut Core,
    ) -> Handle<HalfEdge> {
        let half_edge = HalfEdge::new(
            sibling.boundary().reverse(),
            sibling.curve().clone(),
            start_vertex,
        )
        .insert(core);

        core.layers.geometry.define_half_edge(
            half_edge.clone(),
            core.layers.geometry.of_half_edge(sibling),
        );

        half_edge
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
        core: &mut Core,
    ) -> Handle<HalfEdge> {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let path =
            SurfacePath::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary =
            [arc.start_angle, arc.end_angle].map(|coord| Point::from([coord]));

        let half_edge = HalfEdge::unjoined(boundary, core).insert(core);
        core.layers
            .geometry
            .define_half_edge(half_edge.clone(), HalfEdgeGeometry { path });

        half_edge
    }

    /// Create a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        core: &mut Core,
    ) -> Handle<HalfEdge> {
        let path = SurfacePath::circle_from_center_and_radius(center, radius);
        let boundary =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        let half_edge = HalfEdge::unjoined(boundary, core).insert(core);
        core.layers
            .geometry
            .define_half_edge(half_edge.clone(), HalfEdgeGeometry { path });

        half_edge
    }

    /// Create a line segment
    fn line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        boundary: Option<[Point<1>; 2]>,
        core: &mut Core,
    ) -> Handle<HalfEdge> {
        let boundary =
            boundary.unwrap_or_else(|| [[0.], [1.]].map(Point::from));
        let path = SurfacePath::line_from_points_with_coords(
            boundary.zip_ext(points_surface),
        );

        HalfEdge::unjoined(boundary, core)
            .insert(core)
            .set_geometry(HalfEdgeGeometry { path }, &mut core.layers.geometry)
    }
}

impl BuildHalfEdge for HalfEdge {}
