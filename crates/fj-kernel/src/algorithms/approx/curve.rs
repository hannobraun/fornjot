use std::cmp::max;

use fj_math::{Circle, Point, Scalar};

use crate::objects::{Curve, CurveKind, GlobalCurve, Vertex};

use super::{Approx, Tolerance};

impl Approx for Curve {
    type Approximation = Vec<(Point<2>, Point<3>)>;
    type Params = RangeOnCurve;

    fn approx(
        &self,
        tolerance: Tolerance,
        range: Self::Params,
    ) -> Self::Approximation {
        self.global_form()
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
        let mut points = Vec::new();

        match self.kind() {
            CurveKind::Circle(curve) => {
                approx_circle(curve, range, tolerance, &mut points);
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
fn approx_circle(
    circle: &Circle<3>,
    range: impl Into<RangeOnCurve>,
    tolerance: Tolerance,
    points: &mut Vec<(Point<1>, Point<3>)>,
) {
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

        points.push((point_curve, point_global));
    }
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

#[derive(Clone, Copy)]
pub struct RangeOnCurve {
    pub boundary: [Vertex; 2],
}

impl RangeOnCurve {
    pub fn start(&self) -> Vertex {
        self.boundary[0]
    }

    pub fn end(&self) -> Vertex {
        self.boundary[1]
    }

    pub fn signed_length(&self) -> Scalar {
        (self.end().position() - self.start().position()).t
    }

    pub fn length(&self) -> Scalar {
        self.signed_length().abs()
    }

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
