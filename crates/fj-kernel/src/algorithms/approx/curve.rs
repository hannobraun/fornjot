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

use fj_math::{Point, Scalar};

use crate::objects::{Curve, GlobalCurve};

use super::{Approx, ApproxCache, ApproxPoint, Tolerance};

impl Approx for (&Curve, RangeOnPath) {
    type Approximation = CurveApprox;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        let points = (curve.global_form(), range)
            .approx_with_cache(tolerance, cache)
            .points
            .into_iter()
            .map(|point| {
                let point_surface =
                    curve.path().point_from_path_coords(point.local_form);
                ApproxPoint::new(point_surface, point.global_form)
                    .with_source((*curve, point.local_form))
            });

        CurveApprox::empty().with_points(points)
    }
}

impl Approx for (&GlobalCurve, RangeOnPath) {
    type Approximation = GlobalCurveApprox;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        if let Some(approx) = cache.global_curve(curve) {
            return approx;
        }

        let points = curve.path().approx(range, tolerance);
        cache.insert_global_curve(curve, GlobalCurveApprox { points })
    }
}

/// The range on which a curve should be approximated
#[derive(Clone, Copy, Debug)]
pub struct RangeOnPath {
    boundary: [Point<1>; 2],
    is_reversed: bool,
}

impl RangeOnPath {
    /// Construct an instance of `RangeOnCurve`
    ///
    /// Ranges are normalized on construction, meaning that the order of
    /// vertices passed to this constructor does not influence the range that is
    /// constructed.
    ///
    /// This is done to prevent bugs during mesh construction: The curve
    /// approximation code is regularly faced with ranges that are reversed
    /// versions of each other. This can lead to slightly different
    /// approximations, which in turn leads to the aforementioned invalid
    /// meshes.
    ///
    /// The caller can use `is_reversed` to determine, if the range was reversed
    /// during normalization, to adjust the approximation accordingly.
    pub fn new(boundary: [impl Into<Point<1>>; 2]) -> Self {
        let [a, b] = boundary.map(Into::into);

        let (boundary, is_reversed) = if a < b {
            ([a, b], false)
        } else {
            ([b, a], true)
        };

        Self {
            boundary,
            is_reversed,
        }
    }

    /// Indicate whether the range was reversed during normalization
    pub fn is_reversed(&self) -> bool {
        self.is_reversed
    }

    /// Access the start of the range
    pub fn start(&self) -> Point<1> {
        self.boundary[0]
    }

    /// Access the end of the range
    pub fn end(&self) -> Point<1> {
        self.boundary[1]
    }

    /// Compute the signed length of the range
    pub fn signed_length(&self) -> Scalar {
        (self.end() - self.start()).t
    }

    /// Compute the absolute length of the range
    pub fn length(&self) -> Scalar {
        self.signed_length().abs()
    }

    /// Compute the direction of the range
    ///
    /// Returns a [`Scalar`] that is zero or +/- one.
    pub fn direction(&self) -> Scalar {
        self.signed_length().sign()
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

/// An approximation of a [`GlobalCurve`]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalCurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<1>>,
}
