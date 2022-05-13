use std::cmp::max;

use fj_math::{Circle, Scalar};

use crate::geometry::{self, Curve};

use super::Tolerance;

/// Compute an approximation of the curve
///
/// `tolerance` defines how far the approximation is allowed to deviate from the
/// actual edge.
///
/// # Implementation Note
///
/// This only works as it is, because edges are severely limited and don't
/// define which section of the curve they inhabit. Once they do that, we need
/// an `approximate_between(a, b)` method instead, where `a` and `b` are the
/// vertices that bound the edge on the curve.
///
/// The `approximate_between` methods of the curves then need to make sure to
/// only return points in between those vertices, not the vertices themselves.
pub fn approx_curve(
    curve: &Curve,
    tolerance: Tolerance,
    out: &mut Vec<geometry::Point<1>>,
) {
    match curve {
        Curve::Circle(curve) => approx_circle(curve, tolerance, out),
        Curve::Line(_) => {}
    }
}

/// Approximate the circle
///
/// `tolerance` specifies how much the approximation is allowed to deviate
/// from the circle.
pub fn approx_circle(
    circle: &Circle<3>,
    tolerance: Tolerance,
    out: &mut Vec<geometry::Point<1>>,
) {
    let radius = circle.a.magnitude();

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    let n = number_of_vertices_for_circle(tolerance, radius);

    for i in 0..n {
        let angle = Scalar::PI * 2. / n as f64 * i as f64;
        let point = circle.point_from_circle_coords([angle]);
        out.push(geometry::Point::new([angle], point));
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

    use crate::algorithms::Tolerance;

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
