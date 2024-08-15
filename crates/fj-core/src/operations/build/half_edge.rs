use fj_math::{Arc, Point, Scalar};

use crate::{
    geometry::{CurveBoundary, LocalCurveGeom, Path},
    operations::{geometry::UpdateCurveGeometry, insert::Insert},
    storage::Handle,
    topology::{Curve, HalfEdge, Surface, Vertex},
    Core,
};

/// Build a [`HalfEdge`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildHalfEdge {
    /// Create a half-edge that is not joined to a sibling
    fn unjoined(core: &mut Core) -> HalfEdge {
        let curve = Curve::new().insert(core);
        let start_vertex = Vertex::new().insert(core);

        HalfEdge::new(curve, start_vertex)
    }

    /// Create a half-edge from its sibling
    fn from_sibling(
        sibling: &Handle<HalfEdge>,
        start_vertex: Handle<Vertex>,
        core: &mut Core,
    ) -> Handle<HalfEdge> {
        HalfEdge::new(sibling.curve().clone(), start_vertex).insert(core)
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
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> (Handle<HalfEdge>, CurveBoundary<Point<1>>) {
        let angle_rad = angle_rad.into();
        if angle_rad <= -Scalar::TAU || angle_rad >= Scalar::TAU {
            panic!("arc angle must be in the range (-2pi, 2pi) radians");
        }

        let arc = Arc::from_endpoints_and_angle(start, end, angle_rad);

        let path = Path::circle_from_center_and_radius(arc.center, arc.radius);
        let boundary = CurveBoundary {
            inner: [arc.start_angle, arc.end_angle]
                .map(|coord| Point::from([coord])),
        };

        let half_edge = HalfEdge::unjoined(core).insert(core);

        core.layers.geometry.define_curve(
            half_edge.curve().clone(),
            surface,
            LocalCurveGeom { path },
        );

        (half_edge, boundary)
    }

    /// Create a line segment
    fn line_segment(
        points_surface: [impl Into<Point<2>>; 2],
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> (Handle<HalfEdge>, CurveBoundary<Point<1>>) {
        let boundary = CurveBoundary::default();

        let half_edge = HalfEdge::unjoined(core).insert(core);

        half_edge.curve().clone().make_line_on_surface(
            points_surface,
            boundary,
            surface.clone(),
            &mut core.layers.geometry,
        );

        (half_edge, boundary)
    }
}

impl BuildHalfEdge for HalfEdge {}
