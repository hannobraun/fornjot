//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left out, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to deal with duplicate vertices.

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{HalfEdge, Surface},
};

use super::{
    curve::CurveApproxCache, vertex::VertexApproxCache, Approx, ApproxPoint,
    Tolerance,
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

        let start_position_surface =
            geometry.of_half_edge(half_edge).start_position(
                &geometry
                    .of_curve(half_edge.curve())
                    .unwrap()
                    .local_on(surface)
                    .unwrap()
                    .path,
            );
        let start_position =
            match cache.start_position.get(half_edge.start_vertex()) {
                Some(position) => position,
                None => {
                    let position_global = geometry
                        .of_surface(surface)
                        .point_from_surface_coords(start_position_surface);
                    cache.start_position.insert(
                        half_edge.start_vertex().clone(),
                        position_global,
                    )
                }
            };

        let first = ApproxPoint::new(start_position_surface, start_position);

        let rest = {
            let approx = (
                half_edge.curve(),
                surface,
                geometry.of_half_edge(half_edge).boundary,
            )
                .approx_with_cache(
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
