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
//!
//! ## Determinism
//!
//! Path approximation is carefully designed to produce a deterministic result
//! for the combination of a given path and a given tolerance, regardless of
//! what the range is. This is done to prevent invalid meshes from being
//! generated.
//!
//! In specific terms, this means there is an infinite set of points that
//! approximates a path, and that set is deterministic for a given combination
//! of path and tolerance. The range that defines where the path is approximated
//! only influences the result in two ways:
//!
//! 1. It controls which points from the infinite set are actually computed.
//! 2. It defines the order in which the computed points are returned.
//!
//! As a result, path approximation is guaranteed to generate points that can
//! fit together in a valid mesh, no matter which ranges of a path are being
//! approximated, and how many times.

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
}

impl<T> From<[T; 2]> for RangeOnPath
where
    T: Into<Point<1>>,
{
    fn from(boundary: [T; 2]) -> Self {
        Self::new(boundary)
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

    let params = PathApproxParams::for_circle(circle, tolerance);

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    for point_curve in params.points(range) {
        let point_global = circle.point_from_circle_coords(point_curve);
        points.push(ApproxPoint::new(point_curve, point_global));
    }

    if range.is_reversed() {
        points.reverse();
    }

    points
}

struct PathApproxParams {
    increment: Scalar,
}

impl PathApproxParams {
    pub fn for_circle<const D: usize>(
        circle: &Circle<D>,
        tolerance: impl Into<Tolerance>,
    ) -> Self {
        let radius = circle.a().magnitude();

        let num_vertices_to_approx_full_circle = Scalar::max(
            Scalar::PI
                / (Scalar::ONE - (tolerance.into().inner() / radius)).acos(),
            3.,
        )
        .ceil();

        let increment = Scalar::TAU / num_vertices_to_approx_full_circle;

        Self { increment }
    }

    pub fn increment(&self) -> Scalar {
        self.increment
    }

    pub fn points(
        &self,
        range: impl Into<RangeOnPath>,
    ) -> impl Iterator<Item = Point<1>> + '_ {
        use std::iter;

        let range = range.into();

        let [a, b] = range.boundary.map(|point| point.t / self.increment());

        // We can't generate a point exactly at the end of the range as part of
        // the approximation. Make sure we stop one step before that.
        let b = if b.ceil() == b { b - 1. } else { b };

        let start = a.floor() + 1.;
        let end = b - 1.;

        let mut i = start;
        iter::from_fn(move || {
            if i <= end {
                let t = self.increment() * i;
                i += Scalar::ONE;

                Some(Point::from([t]))
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::TAU;

    use fj_math::{Circle, Point, Scalar};

    use crate::algorithms::approx::{path::RangeOnPath, Tolerance};

    use super::PathApproxParams;

    #[test]
    fn increment_for_circle() {
        test_increment(1., 0.5, 3.);
        test_increment(1., 0.1, 7.);
        test_increment(1., 0.01, 23.);

        fn test_increment(
            radius: impl Into<Scalar>,
            tolerance: impl Into<Tolerance>,
            expected_num_vertices: impl Into<Scalar>,
        ) {
            let circle = Circle::from_center_and_radius([0., 0.], radius);
            let params = PathApproxParams::for_circle(&circle, tolerance);

            let expected_increment = Scalar::TAU / expected_num_vertices;
            assert_eq!(params.increment(), expected_increment);
        }
    }

    #[test]
    fn points_for_circle() {
        // Needed to support type inference.
        let empty: [Scalar; 0] = [];

        // At the chosen values, the increment is ~1.25.
        test_path([[0.], [0.]], empty); // empty range
        test_path([[0.], [TAU]], [1., 2., 3.]); // start before first increment
        test_path([[1.], [TAU]], [1., 2., 3.]); // start before first increment
        test_path([[0.], [TAU - 1.]], [1., 2., 3.]); // end after last increment
        test_path([[2.], [TAU]], [2., 3.]); // start after first increment
        test_path([[0.], [TAU - 2.]], [1., 2.]); // end before last increment

        fn test_path(
            range: impl Into<RangeOnPath>,
            expected_coords: impl IntoIterator<Item = impl Into<Scalar>>,
        ) {
            // Choose radius and tolerance such, that we need 5 vertices to
            // approximate a full circle. This is the lowest number that we can
            // still cover all the edge cases with
            let radius = 1.;
            let tolerance = 0.25;

            let circle = Circle::from_center_and_radius([0., 0.], radius);
            let params = PathApproxParams::for_circle(&circle, tolerance);

            let points = params.points(range).collect::<Vec<_>>();

            let expected_points = expected_coords
                .into_iter()
                .map(|i| Point::from([params.increment() * i]))
                .collect::<Vec<_>>();
            assert_eq!(points, expected_points);
        }
    }
}
