//! Curve approximation
//!
//! Since curves are infinite (even circles have an infinite coordinate space,
//! even though they connect to themselves in global coordinates), a range must
//! be provided to approximate them. The approximation then returns points
//! within that range.
//!
//! The boundaries of the range are not included in the approximation. This is
//! done, to give the caller (who knows the boundary anyway) more options on how
//! to further process the approximation.

use std::collections::BTreeMap;

use crate::objects::{Curve, GlobalCurve};

use super::{path::RangeOnPath, Approx, ApproxPoint, Tolerance};

impl Approx for (&Curve, RangeOnPath) {
    type Approximation = CurveApprox;
    type Cache = CurveCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        let cache_key = (*curve.global_form(), range);
        let global_curve_approx = match cache.global_curve(cache_key) {
            Some(approx) => approx,
            None => {
                let approx = (curve.global_form(), range)
                    .approx_with_cache(tolerance, &mut ());
                cache.insert_global_curve(cache_key, approx)
            }
        };

        CurveApprox::empty().with_points(
            global_curve_approx.points.into_iter().map(|point| {
                let point_surface =
                    curve.path().point_from_path_coords(point.local_form);
                ApproxPoint::new(point_surface, point.global_form)
                    .with_source((*curve, point.local_form))
            }),
        )
    }
}

impl Approx for (&GlobalCurve, RangeOnPath) {
    type Approximation = GlobalCurveApprox;
    type Cache = ();

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        let points = (curve.path(), range)
            .approx_with_cache(tolerance, cache)
            .into_iter()
            .map(|(point_curve, point_global)| {
                ApproxPoint::new(point_curve, point_global)
            })
            .collect();

        GlobalCurveApprox { points }
    }
}

/// An approximation of a [`Curve`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<2>>,
}

impl CurveApprox {
    /// Create an empty instance of `CurveApprox`
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// Add points to the approximation
    pub fn with_points(
        mut self,
        points: impl IntoIterator<Item = ApproxPoint<2>>,
    ) -> Self {
        self.points.extend(points);
        self
    }
}

/// A cache for results of an approximation
#[derive(Default)]
pub struct CurveCache {
    inner: BTreeMap<(GlobalCurve, RangeOnPath), GlobalCurveApprox>,
}

impl CurveCache {
    /// Create an empty cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert the approximation of a [`GlobalCurve`]
    pub fn insert_global_curve(
        &mut self,
        key: (GlobalCurve, RangeOnPath),
        approx: GlobalCurveApprox,
    ) -> GlobalCurveApprox {
        self.inner.insert(key, approx.clone());
        approx
    }

    /// Access the approximation for the given [`GlobalCurve`], if available
    pub fn global_curve(
        &self,
        key: (GlobalCurve, RangeOnPath),
    ) -> Option<GlobalCurveApprox> {
        self.inner.get(&key).cloned()
    }
}

/// An approximation of a [`GlobalCurve`]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalCurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<1>>,
}
