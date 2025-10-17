use fj_interop::Tolerance;
use fj_math::{Point, Scalar, Vector};

use crate::{approx::curve::CurveApproxFloating, geometry::curve::CurveGeometry};

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
    ) -> CurveApproxFloating {
        let [a, b] = boundary;
        let direction = (b.t - a.t).sign();

        let [min, max] = if direction.is_positive() {
            [a, b]
        } else {
            [b, a]
        };

        let size_hint = max.t - min.t;
        let increment = self.increment(tolerance, size_hint).t;

        let mut curvature = Vec::new();

        let mut t = (min.t / increment).floor() * increment + increment;
        while t <= (max.t / increment).floor() * increment {
            curvature.push(Point::from([t]));
            t += increment;
        }

        if direction.is_negative() {
            curvature.reverse();
        }

        CurveApproxFloating { curvature }
    }

    fn increment(&self, tolerance: Tolerance, _: Scalar) -> Vector<1> {
        let num_vertices_to_approx_full_circle = Scalar::max(
            Scalar::PI
                / (Scalar::ONE - (tolerance.inner() / self.radius())).acos(),
            3.,
        )
        .ceil();

        let increment = Scalar::TAU / num_vertices_to_approx_full_circle;
        Vector::from([increment])
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
