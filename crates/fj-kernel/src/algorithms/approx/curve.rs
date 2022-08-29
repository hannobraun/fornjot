use std::cmp::max;

use fj_math::{Circle, Point, Scalar};

use crate::objects::{Curve, CurveKind, GlobalCurve};

use super::{Approx, Tolerance};

impl Approx for Curve {
    type Approximation = Vec<(Point<2>, Point<3>)>;
    type Params = RangeOnCurve;

    fn approx(
        &self,
        tolerance: Tolerance,
        range: Self::Params,
    ) -> Self::Approximation {
        self.global()
            .approx(tolerance, range)
            .into_iter()
            .map(|(point_curve, point_global)| {
                let point_surface =
                    self.kind().point_from_curve_coords(point_curve);
                (point_surface, point_global)
            })
            .collect()
    }
}

impl Approx for GlobalCurve {
    type Approximation = Vec<(Point<1>, Point<3>)>;
    type Params = RangeOnCurve;

    fn approx(
        &self,
        tolerance: Tolerance,
        range: Self::Params,
    ) -> Self::Approximation {
        match self.kind() {
            CurveKind::Circle(curve) => approx_circle(curve, range, tolerance),
            CurveKind::Line(_) => vec![range.start()],
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
) -> Vec<(Point<1>, Point<3>)> {
    let radius = circle.a().magnitude();
    let range = range.into();

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    let n = number_of_vertices_for_circle(tolerance, radius, range.length());

    let mut points = Vec::new();
    points.push(range.start());

    for i in 1..n {
        let angle = range.start().0.t
            + (Scalar::TAU / n as f64 * i as f64) * range.direction();

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

pub struct RangeOnCurve {
    pub boundary: [(Point<1>, Point<3>); 2],
}

impl RangeOnCurve {
    fn start(&self) -> (Point<1>, Point<3>) {
        self.boundary[0]
    }

    fn end(&self) -> (Point<1>, Point<3>) {
        self.boundary[1]
    }

    fn signed_length(&self) -> Scalar {
        (self.end().0 - self.start().0).t
    }

    fn length(&self) -> Scalar {
        self.signed_length().abs()
    }

    fn direction(&self) -> Scalar {
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
