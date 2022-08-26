use std::cmp::max;

use fj_math::{Circle, Point, Scalar};

use crate::objects::{Curve, CurveKind, GlobalCurve};

use super::{Approx, Tolerance};

impl Approx for Curve {
    type Approximation = Vec<(Point<1>, Point<3>)>;
    type Params = ();

    fn approx(
        &self,
        tolerance: Tolerance,
        (): Self::Params,
    ) -> Self::Approximation {
        self.global().approx(tolerance, ())
    }
}

impl Approx for GlobalCurve {
    type Approximation = Vec<(Point<1>, Point<3>)>;
    type Params = ();

    /// Approximate the global curve
    ///
    /// # Implementation Note
    ///
    /// This only works as-is, because only circles need to be approximated
    /// right now and because only edges that are full circles are supported, as
    /// opposed to edges that only inhabit part of the circle.
    ///
    /// To support that, we will need additional information here, to define
    /// between which points the curve needs to be approximated.
    fn approx(
        &self,
        tolerance: Tolerance,
        (): Self::Params,
    ) -> Self::Approximation {
        match self.kind() {
            CurveKind::Circle(curve) => {
                approx_circle(curve, [[Scalar::ZERO], [Scalar::TAU]], tolerance)
            }
            CurveKind::Line(_) => Vec::new(),
        }
    }
}

/// Approximate a circle
///
/// `tolerance` specifies how much the approximation is allowed to deviate
/// from the circle.
pub fn approx_circle(
    circle: &Circle<3>,
    between: [impl Into<Point<1>>; 2],
    tolerance: Tolerance,
) -> Vec<(Point<1>, Point<3>)> {
    let mut points = Vec::new();

    let radius = circle.a().magnitude();

    let [start, end] = between.map(Into::into);
    let range = (end - start).t;

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    let n = number_of_vertices_for_circle(tolerance, radius, range.abs());

    for i in 0..n {
        let angle =
            start.t + (Scalar::TAU / n as f64 * i as f64) * range.sign();

        let point_curve = Point::from([angle]);
        let point_global = circle.point_from_circle_coords(point_curve);

        points.push((point_curve, point_global));
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
