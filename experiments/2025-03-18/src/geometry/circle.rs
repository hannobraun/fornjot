use std::iter;

use fj_interop::Tolerance;
use fj_math::{Point, Scalar, Sign, Vector};

use crate::{approx::curve::CurveApprox, geometry::curve::CurveGeometry};

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub a: Vector<3>,
    pub b: Vector<3>,
}

impl Circle {
    pub fn radius(&self) -> Scalar {
        self.a.magnitude()
    }

    pub fn vector_from_local_point(
        &self,
        point: impl Into<Point<1>>,
    ) -> Vector<3> {
        let angle = point.into().t;
        let (sin, cos) = angle.sin_cos();

        self.a * cos + self.b * sin - self.a
    }

    pub fn project_vector(&self, vector: impl Into<Vector<3>>) -> Point<1> {
        let vector = self.a + vector.into();

        let [a, b] =
            [&self.a, &self.b].map(|v| vector.scalar_projection_onto(v));

        let atan = Scalar::atan2(b, a);
        let coord = if atan >= Scalar::ZERO {
            atan
        } else {
            atan + Scalar::TAU
        };

        Point::from([coord])
    }
}

impl CurveGeometry for Circle {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry> {
        Box::new(*self)
    }

    fn vector_from_local_point(&self, point: Point<1>) -> Vector<3> {
        self.vector_from_local_point(point)
    }

    fn project_vector(&self, vector: Vector<3>) -> Point<1> {
        self.project_vector(vector)
    }

    fn flip(&self) -> Box<dyn CurveGeometry> {
        Box::new(Circle {
            a: self.a,
            b: -self.b,
        })
    }

    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> CurveApprox {
        let curvature = CircleApproxParams::new(self.radius(), tolerance)
            .approx_circle(boundary)
            .collect();

        CurveApprox { curvature }
    }
}

/// # Approximation parameters for a circle
#[derive(Debug)]
pub struct CircleApproxParams {
    increment: Vector<1>,
}

impl CircleApproxParams {
    /// # Compute the approximation parameters for a given circle and tolerance
    pub fn new(
        radius: impl Into<Scalar>,
        tolerance: impl Into<Tolerance>,
    ) -> Self {
        let num_vertices_to_approx_full_circle = Scalar::max(
            Scalar::PI
                / (Scalar::ONE - (tolerance.into().inner() / radius)).acos(),
            3.,
        )
        .ceil();

        let t = Scalar::TAU / num_vertices_to_approx_full_circle;
        let increment = Vector::from([t]);

        Self { increment }
    }

    /// # Generate points to approximate the circle within a given boundary
    pub fn approx_circle(
        &self,
        boundary: [Point<1>; 2],
    ) -> impl Iterator<Item = Point<1>> + '_ {
        // The boundary, in units of the increment.
        let [a, b] = boundary.map(|point| point.t / self.increment.t);

        let direction = (b - a).sign();
        let [min, max] = if a < b { [a, b] } else { [b, a] };

        // We can't generate a point exactly at the boundaries of the range as
        // part of the approximation. Make sure we stay inside the range.
        //
        // `min` and `max` are in units of the increment, so adding or
        // subtracting `1` adds or subtracts one increment.
        let min = min.floor() + 1.;
        let max = max.ceil() - 1.;

        let [start, end] = match direction {
            Sign::Negative => [max, min],
            Sign::Positive | Sign::Zero => [min, max],
        };

        let mut i = start;
        iter::from_fn(move || {
            let is_finished = match direction {
                Sign::Negative => i < end,
                Sign::Positive | Sign::Zero => i > end,
            };

            if is_finished {
                return None;
            }

            let t = self.increment.t * i;
            i += direction.to_scalar();

            Some(Point::from([t]))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    use approx::assert_abs_diff_eq;
    use fj_math::Vector;

    use crate::geometry::Circle;

    #[test]
    fn vector_from_local_point() {
        let circle = Circle {
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., 1., 0.]),
        };

        assert_abs_diff_eq!(
            circle.vector_from_local_point([0.]),
            Vector::from([0., 0., 0.]),
        );
        assert_abs_diff_eq!(
            circle.vector_from_local_point([FRAC_PI_4]),
            Vector::from([2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.]) - circle.a,
        );
        assert_abs_diff_eq!(
            circle.vector_from_local_point([FRAC_PI_2]),
            Vector::from([-1., 1., 0.]),
        );
        assert_abs_diff_eq!(
            circle.vector_from_local_point([FRAC_PI_4 * 3.]),
            Vector::from([-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.])
                - circle.a,
        );
        assert_abs_diff_eq!(
            circle.vector_from_local_point([PI]),
            Vector::from([-2., 0., 0.]),
        );
        assert_abs_diff_eq!(
            circle.vector_from_local_point([FRAC_PI_4 * 5.]),
            Vector::from([-2_f64.sqrt() / 2., -2_f64.sqrt() / 2., 0.])
                - circle.a,
        );
        assert_abs_diff_eq!(
            circle.vector_from_local_point([FRAC_PI_2 * 3.]),
            Vector::from([-1., -1., 0.]),
        );
        assert_abs_diff_eq!(
            circle.vector_from_local_point([FRAC_PI_4 * 7.]),
            Vector::from([2_f64.sqrt() / 2., -2_f64.sqrt() / 2., 0.])
                - circle.a,
        );
    }
}
