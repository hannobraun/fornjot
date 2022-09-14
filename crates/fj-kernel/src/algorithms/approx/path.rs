//! # Path approximation
//!
//! Since paths are infinite (even circles have an infinite coordinate space,
//! even though they connect to themselves in global coordinates), a range must
//! be provided to approximate them. The approximation then returns points
//! within that range.
//!
//! The boundaries of the range are not included in the approximation. This is
//! done, to give the caller (who knows the boundary anyway) more options on how
//! to further process the approximation.

use fj_math::{Circle, Point, Scalar};

use crate::path::GlobalPath;

use super::{Approx, ApproxCache, ApproxPoint, Tolerance};

impl Approx for (GlobalPath, RangeOnPath) {
    type Approximation = GlobalPathApprox;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        _: &mut ApproxCache,
    ) -> Self::Approximation {
        let (path, range) = self;

        let points = match path {
            GlobalPath::Circle(circle) => {
                approx_circle(&circle, range, tolerance.into())
            }
            GlobalPath::Line(_) => vec![],
        };

        GlobalPathApprox { points }
    }
}

/// An approximation of a [`GlobalPath`]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalPathApprox {
    points: Vec<ApproxPoint<1>>,
}

impl GlobalPathApprox {
    /// Access the points that approximate the path
    pub fn points(&self) -> impl Iterator<Item = ApproxPoint<1>> + '_ {
        self.points.iter().cloned()
    }
}

/// Approximate a circle
///
/// `tolerance` specifies how much the approximation is allowed to deviate
/// from the circle.
fn approx_circle(
    circle: &Circle<3>,
    range: impl Into<RangeOnPath>,
    tolerance: Tolerance,
) -> Vec<ApproxPoint<1>> {
    let mut points = Vec::new();

    let range = range.into();

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    let n = number_of_vertices_for_circle(
        tolerance,
        circle.radius(),
        range.length(),
    );

    for i in 1..n {
        let angle = range.start().t
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
    let n = (Scalar::PI / (Scalar::ONE - (tolerance.inner() / radius)).acos())
        .max(3.);

    (n / (Scalar::TAU / range)).ceil().into_u64()
}

/// The range on which a path should be approximated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
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

    /// Access the boundary of the range
    pub fn boundary(&self) -> [Point<1>; 2] {
        self.boundary
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

#[cfg(test)]
mod tests {
    use fj_math::Scalar;

    use crate::algorithms::approx::Tolerance;

    #[test]
    fn number_of_vertices_for_circle() {
        verify_result(50., 100., Scalar::TAU, 3);
        verify_result(50., 100., Scalar::PI, 2);
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
