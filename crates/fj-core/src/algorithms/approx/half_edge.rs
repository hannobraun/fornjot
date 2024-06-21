//! Half-edge approximation
//!
//! See [`HalfEdgeApprox`].

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{HalfEdge, Surface},
};

use super::{
    curve::{approx_curve_with_cache, CurveApproxCache},
    vertex::{approx_vertex, VertexApproxCache},
    Approx, ApproxPoint, Tolerance,
};

impl Approx for (&Handle<HalfEdge>, &Handle<Surface>) {
    type Approximation = HalfEdgeApprox;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation {
        let (half_edge, surface) = self;
        let tolerance = tolerance.into();

        let boundary = geometry.of_half_edge(half_edge).boundary;

        let start_position_surface =
            geometry.of_half_edge(half_edge).start_position(
                &geometry
                    .of_curve(half_edge.curve())
                    .unwrap()
                    .local_on(surface)
                    .unwrap()
                    .path,
            );
        let first = approx_vertex(
            half_edge.start_vertex().clone(),
            surface,
            start_position_surface,
            &mut cache.start_position,
            geometry,
        );

        let rest = {
            let approx = approx_curve_with_cache(
                half_edge.curve(),
                surface,
                boundary,
                tolerance,
                &mut cache.curve,
                geometry,
            );

            approx.points.into_iter().map(|point| {
                let point_surface = geometry
                    .of_curve(half_edge.curve())
                    .unwrap()
                    .local_on(surface)
                    .unwrap()
                    .path
                    .point_from_path_coords(point.local_form);

                ApproxPoint::new(point_surface, point.global_form)
            })
        };

        let mut points = vec![first];
        points.extend(rest);

        HalfEdgeApprox { points }
    }
}

/// An approximation of a [`HalfEdge`]
///
/// The approximation of a half-edge is its first vertex, combined with the
/// approximation of its curve. The second vertex is left out, as half-edge
/// approximations are usually used to build cycle approximations, and this way,
/// the caller doesn't have to deal with duplicate vertices.
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdgeApprox {
    /// The points that approximate the half-edge
    pub points: Vec<ApproxPoint<2>>,
}

/// Cache for half-edge approximations
#[derive(Default)]
pub struct HalfEdgeApproxCache {
    start_position: VertexApproxCache,
    curve: CurveApproxCache,
}
