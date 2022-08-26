use std::cmp::max;

use fj_math::{Circle, Point, Scalar};

use crate::objects::{Curve, CurveKind, GlobalCurve};

use super::{Approx, Local, Tolerance};

impl Approx for Curve {
    type Approximation = Vec<Local<Point<1>>>;

    fn approx(&self, tolerance: Tolerance) -> Self::Approximation {
        self.global().approx(tolerance)
    }
}

impl Approx for GlobalCurve {
    type Approximation = Vec<Local<Point<1>>>;

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
    fn approx(&self, tolerance: Tolerance) -> Self::Approximation {
        let mut points = Vec::new();

        match self.kind() {
            CurveKind::Circle(curve) => {
                approx_circle(curve, tolerance, &mut points)
            }
            CurveKind::Line(_) => {}
        }

        points
    }
}

/// Approximate a circle
///
/// `tolerance` specifies how much the approximation is allowed to deviate
/// from the circle.
pub fn approx_circle(
    circle: &Circle<3>,
    tolerance: Tolerance,
    out: &mut Vec<Local<Point<1>>>,
) {
    let radius = circle.a().magnitude();

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    let n = number_of_vertices_for_circle(tolerance, radius);

    for i in 0..n {
        let angle = Scalar::TAU / n as f64 * i as f64;
        let point = circle.point_from_circle_coords([angle]);
        out.push(Local::new([angle], point));
    }
}

fn number_of_vertices_for_circle(tolerance: Tolerance, radius: Scalar) -> u64 {
    let n = (Scalar::PI / (Scalar::ONE - (tolerance.inner() / radius)).acos())
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
        verify_result(50., 100., 3);
        verify_result(10., 100., 7);
        verify_result(1., 100., 23);

        fn verify_result(
            tolerance: impl Into<Tolerance>,
            radius: impl Into<Scalar>,
            n: u64,
        ) {
            let tolerance = tolerance.into();
            let radius = radius.into();

            assert_eq!(
                n,
                super::number_of_vertices_for_circle(tolerance, radius)
            );

            assert!(calculate_error(radius, n) <= tolerance.inner());
            if n > 3 {
                assert!(calculate_error(radius, n - 1) >= tolerance.inner());
            }
        }

        fn calculate_error(radius: Scalar, n: u64) -> Scalar {
            radius - radius * (Scalar::PI / Scalar::from_u64(n)).cos()
        }
    }
}
