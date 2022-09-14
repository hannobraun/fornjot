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

use std::cmp::max;

use fj_math::{Circle, Point, Scalar};

use crate::{
    objects::{Curve, GlobalCurve, Vertex},
    path::GlobalPath,
};

use super::{Approx, ApproxCache, ApproxPoint, Tolerance};

impl Approx for (&Curve, RangeOnCurve) {
    type Approximation = CurveApprox;

    fn approx_with_cache(
        self,
        tolerance: Tolerance,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        let points = (curve.global_form(), range)
            .approx_with_cache(tolerance, cache)
            .into_iter()
            .map(|point| {
                let point_surface =
                    curve.path().point_from_path_coords(point.local_form);
                ApproxPoint::new(point_surface, point.global_form)
                    .with_source((*curve, point.local_form))
            })
            .collect();

        CurveApprox { points }
    }
}

impl Approx for (&GlobalCurve, RangeOnCurve) {
    type Approximation = Vec<ApproxPoint<1>>;

    fn approx_with_cache(
        self,
        tolerance: Tolerance,
        _: &mut ApproxCache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        match curve.path() {
            GlobalPath::Circle(circle) => {
                approx_circle(&circle, range, tolerance)
            }
            GlobalPath::Line(_) => vec![],
        }
    }
}

/// Approximate a circle
///
/// `tolerance` specifies how much the approximation is allowed to deviate
/// from the circle.
fn approx_circle(
    circle: &Circle<3>,
    range: impl Into<RangeOnCurve>,
    tolerance: Tolerance,
) -> Vec<ApproxPoint<1>> {
    let mut points = Vec::new();

    let radius = circle.a().magnitude();
    let range = range.into();

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    let n = number_of_vertices_for_circle(tolerance, radius, range.length());

    for i in 1..n {
        let angle = range.start().position().t
            + (Scalar::TAU / n as f64 * i as f64) * range.direction();

        let point_curve = Point::from([angle]);
        let point_global = circle.point_from_circle_coords(point_curve);

        points.push(ApproxPoint::new(point_curve, point_global));
    }

    if range.is_reversed() {
        points.reverse();
    }

    points
}

fn number_of_vertices_for_circle(
    tolerance: Tolerance,
    radius: Scalar,
    range: Scalar,
) -> u64 {
    let n = (range / (Scalar::ONE - (tolerance.inner() / radius)).acos() / 2.)
        .ceil()
        .into_u64();

    max(n, 3)
}

/// The range on which a curve should be approximated
#[derive(Clone, Copy, Debug)]
pub struct RangeOnCurve {
    boundary: [Vertex; 2],
    is_reversed: bool,
}

impl RangeOnCurve {
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
    pub fn new([a, b]: [Vertex; 2]) -> Self {
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
    pub fn start(&self) -> Vertex {
        self.boundary[0]
    }

    /// Access the end of the range
    pub fn end(&self) -> Vertex {
        self.boundary[1]
    }

    /// Compute the signed length of the range
    pub fn signed_length(&self) -> Scalar {
        (self.end().position() - self.start().position()).t
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

#[cfg(test)]
mod tests {
    use fj_math::Scalar;

    use crate::algorithms::approx::Tolerance;

    #[test]
    fn number_of_vertices_for_circle() {
        verify_result(50., 100., Scalar::TAU, 3);
        verify_result(50., 100., Scalar::PI, 3);
        verify_result(10., 100., Scalar::TAU, 7);
        verify_result(10., 100., Scalar::PI, 4);
        verify_result(1., 100., Scalar::TAU, 23);
        verify_result(1., 100., Scalar::PI, 12);

        fn verify_result(
            tolerance: impl Into<Tolerance>,
            radius: impl Into<Scalar>,
            range: impl Into<Scalar>,
            n: u64,
        ) {
            let tolerance = tolerance.into();
            let radius = radius.into();
            let range = range.into();

            assert_eq!(
                n,
                super::number_of_vertices_for_circle(tolerance, radius, range)
            );

            assert!(calculate_error(radius, range, n) <= tolerance.inner());
            if n > 3 {
                assert!(
                    calculate_error(radius, range, n - 1) >= tolerance.inner()
                );
            }
        }

        fn calculate_error(radius: Scalar, range: Scalar, n: u64) -> Scalar {
            radius - radius * (range / Scalar::from_u64(n) / 2.).cos()
        }
    }
}
