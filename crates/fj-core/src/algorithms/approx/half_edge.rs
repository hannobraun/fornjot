//! Half-edge approximation
//!
//! See [`HalfEdgeApprox`].

use std::iter;

use crate::{
    approx::Tolerance,
    geometry::{CurveBoundary, Geometry},
    math::Point,
    storage::Handle,
    topology::{HalfEdge, Surface},
};

use super::{
    ApproxPoint,
    curve::{CurveApproxCache, approx_curve_with_cache},
};

/// Approximate the provided half-edge
pub fn approx_half_edge(
    half_edge: &Handle<HalfEdge>,
    surface: &Handle<Surface>,
    start: ApproxPoint<1>,
    boundary: CurveBoundary<Point<1>>,
    tolerance: impl Into<Tolerance>,
    cache: &mut CurveApproxCache,
    geometry: &Geometry,
) -> HalfEdgeApprox {
    let tolerance = tolerance.into();

    let rest = approx_curve_with_cache(
        half_edge.curve(),
        surface,
        boundary,
        tolerance,
        cache,
        geometry,
    );

    let points = iter::once(start)
        .chain(rest.points)
        .map(|point| {
            let point_surface = geometry
                .of_curve(half_edge.curve())
                .unwrap()
                .local_on(surface)
                .unwrap()
                .path
                .point_from_path_coords(point.local_form);

            ApproxPoint::new(point_surface, point.global_form)
        })
        .collect();

    HalfEdgeApprox { points }
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
