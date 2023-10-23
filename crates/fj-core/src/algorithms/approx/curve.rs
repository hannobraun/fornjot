//! Curve approximation

use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, GlobalPath, SurfacePath},
    objects::{Curve, Surface},
    storage::{Handle, HandleWrapper},
};

use super::{Approx, ApproxPoint, Tolerance};

impl Approx
    for (
        &Handle<Curve>,
        SurfacePath,
        &Surface,
        CurveBoundary<Point<1>>,
    )
{
    type Approximation = CurveApprox;
    type Cache = CurveApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (curve, surface_path, surface, boundary) = self;

        match cache.get(curve, boundary) {
            Some(approx) => approx,
            None => {
                let approx =
                    approx_curve(&surface_path, surface, boundary, tolerance);

                cache.insert(curve.clone(), boundary, approx)
            }
        }
    }
}

pub(super) fn approx_curve(
    path: &SurfacePath,
    surface: &Surface,
    boundary: CurveBoundary<Point<1>>,
    tolerance: impl Into<Tolerance>,
) -> CurveApprox {
    // There are different cases of varying complexity. Circles are the hard
    // part here, as they need to be approximated, while lines don't need to be.
    //
    // This will probably all be unified eventually, as `SurfacePath` and
    // `GlobalPath` grow APIs that are better suited to implementing this code
    // in a more abstract way.
    let points = match (path, surface.geometry().u) {
        (SurfacePath::Circle(_), GlobalPath::Circle(_)) => {
            todo!(
                "Approximating a circle on a curved surface not supported yet."
            )
        }
        (SurfacePath::Circle(_), GlobalPath::Line(_)) => {
            (path, boundary)
                .approx_with_cache(tolerance, &mut ())
                .into_iter()
                .map(|(point_curve, point_surface)| {
                    // We're throwing away `point_surface` here, which is a bit
                    // weird, as we're recomputing it later (outside of this
                    // function).
                    //
                    // It should be fine though:
                    //
                    // 1. We're throwing this version away, so there's no danger
                    //    of inconsistency between this and the later version.
                    // 2. This version should have been computed using the same
                    //    path and parameters and the later version will be, so
                    //    they should be the same anyway.
                    // 3. Not all other cases handled in this function have a
                    //    surface point available, so it needs to be computed
                    //    later anyway, in the general case.

                    let point_global = surface
                        .geometry()
                        .point_from_surface_coords(point_surface);
                    (point_curve, point_global)
                })
                .collect()
        }
        (SurfacePath::Line(line), _) => {
            let range_u =
                CurveBoundary::from(boundary.inner.map(|point_curve| {
                    [path.point_from_path_coords(point_curve).u]
                }));

            let approx_u = (surface.geometry().u, range_u)
                .approx_with_cache(tolerance, &mut ());

            let mut points = Vec::new();
            for (u, _) in approx_u {
                let t = (u.t - line.origin().u) / line.direction().u;
                let point_surface = path.point_from_path_coords([t]);
                let point_global =
                    surface.geometry().point_from_surface_coords(point_surface);
                points.push((u, point_global));
            }

            points
        }
    };

    let points = points
        .into_iter()
        .map(|(point_curve, point_global)| {
            ApproxPoint::new(point_curve, point_global)
        })
        .collect();
    CurveApprox { points }
}

/// Approximation of [`Curve`], within a specific boundary
#[derive(Clone)]
pub struct CurveApprox {
    /// The points that approximate the curve within the boundary
    pub points: Vec<ApproxPoint<1>>,
}

impl CurveApprox {
    fn reverse(mut self) -> Self {
        self.points.reverse();
        self
    }
}

/// Cache for curve approximations
#[derive(Default)]
pub struct CurveApproxCache {
    inner:
        BTreeMap<(HandleWrapper<Curve>, CurveBoundary<Point<1>>), CurveApprox>,
}

impl CurveApproxCache {
    /// Get an approximated curve from the cache
    pub fn get(
        &self,
        handle: &Handle<Curve>,
        boundary: CurveBoundary<Point<1>>,
    ) -> Option<CurveApprox> {
        let handle = HandleWrapper::from(handle.clone());

        if let Some(approx) = self.inner.get(&(handle.clone(), boundary)) {
            return Some(approx.clone());
        }
        if let Some(approx) = self.inner.get(&(handle, boundary.reverse())) {
            return Some(approx.clone().reverse());
        }

        None
    }

    /// Insert an approximated curve into the cache
    pub fn insert(
        &mut self,
        handle: Handle<Curve>,
        boundary: CurveBoundary<Point<1>>,
        approx: CurveApprox,
    ) -> CurveApprox {
        let handle = HandleWrapper::from(handle);
        self.inner
            .insert((handle, boundary), approx.clone())
            .unwrap_or(approx)
    }
}
