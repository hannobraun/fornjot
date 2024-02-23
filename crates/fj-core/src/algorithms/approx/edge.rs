//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left out, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to deal with duplicate vertices.

use crate::{
    objects::{HalfEdge, Surface},
    Core,
};

use super::{
    curve::CurveApproxCache, vertex::VertexApproxCache, Approx, ApproxPoint,
    Tolerance,
};

impl Approx for (&HalfEdge, &Surface) {
    type Approximation = HalfEdgeApprox;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        core: &mut Core,
    ) -> Self::Approximation {
        let (edge, surface) = self;
        let tolerance = tolerance.into();

        let start_position_surface = edge.start_position();
        let start_position = match cache.start_position.get(edge.start_vertex())
        {
            Some(position) => position,
            None => {
                let position_global = surface
                    .geometry()
                    .point_from_surface_coords(start_position_surface);
                cache
                    .start_position
                    .insert(edge.start_vertex().clone(), position_global)
            }
        };

        let first = ApproxPoint::new(start_position_surface, start_position);

        let rest = {
            let approx = (
                edge.curve(),
                edge.path(),
                &surface.geometry(),
                edge.boundary(),
            )
                .approx_with_cache(
                    tolerance,
                    &mut cache.curve,
                    core,
                );

            approx.points.into_iter().map(|point| {
                let point_surface =
                    edge.path().point_from_path_coords(point.local_form);

                ApproxPoint::new(point_surface, point.global_form)
            })
        };

        let mut points = vec![first];
        points.extend(rest);

        HalfEdgeApprox { points }
    }
}

/// An approximation of a [`HalfEdge`]
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
