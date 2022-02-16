use std::f64::consts::PI;

use nalgebra::{point, vector};

use crate::kernel::math::{Point, Transform, Vector};

/// A circle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    /// The center point of the circle
    pub center: Point<3>,

    /// The radius of the circle
    ///
    /// The radius is represented by a vector that points from the center to the
    /// circumference. The point on the circumference that it points to defines
    /// the origin of the circle's 1-dimensional curve coordinate system.
    pub radius: Vector<2>,
}

impl Circle {
    /// Access the origin of the curve's coordinate system
    pub fn origin(&self) -> Point<3> {
        self.center
    }

    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        let radius = vector![self.radius.x, self.radius.y, 0.];

        Self {
            center: transform.transform_point(&self.center),
            radius: transform.transform_vector(&radius).xy(),
        }
    }

    /// Convert a point in model coordinates to curve coordinates
    ///
    /// Converts the provided point into curve coordinates between `0.`
    /// (inclusive) and `PI * 2.` (exclusive).
    ///
    /// Projects the point onto the circle before computing curve coordinate,
    /// ignoring the radius. This is done to make this method robust against
    /// floating point accuracy issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the curve, intentional or not, will not result in an
    /// error.
    pub fn point_model_to_curve(&self, point: &Point<3>) -> Point<1> {
        let v = point - self.center;
        let atan = f64::atan2(v.y, v.x);
        let coord = if atan >= 0. { atan } else { atan + PI * 2. };
        point![coord]
    }

    /// Convert a point on the curve into model coordinates
    pub fn point_curve_to_model(&self, point: &Point<1>) -> Point<3> {
        self.center + self.vector_curve_to_model(&point.coords)
    }

    /// Convert a vector on the curve into model coordinates
    pub fn vector_curve_to_model(&self, point: &Vector<1>) -> Vector<3> {
        let radius = self.radius.magnitude();
        let angle = point.x;

        let (sin, cos) = angle.sin_cos();

        let x = cos * radius;
        let y = sin * radius;

        vector![x, y, 0.]
    }

    pub fn approx(&self, tolerance: f64, out: &mut Vec<Point<3>>) {
        let radius = self.radius.magnitude();

        // To approximate the circle, we use a regular polygon for which
        // the circle is the circumscribed circle. The `tolerance`
        // parameter is the maximum allowed distance between the polygon
        // and the circle. This is the same as the difference between
        // the circumscribed circle and the incircle.

        let n = Circle::number_of_vertices(tolerance, radius);

        for i in 0..n {
            let angle = 2. * PI / n as f64 * i as f64;
            let point = self.point_curve_to_model(&point![angle]);
            out.push(point);
        }
    }

    fn number_of_vertices(tolerance: f64, radius: f64) -> u64 {
        assert!(tolerance > 0.);
        if tolerance > radius / 2. {
            3
        } else {
            (PI / (1. - (tolerance / radius)).acos()).ceil() as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use nalgebra::{point, vector};

    use super::Circle;

    #[test]
    fn point_model_to_curve() {
        let circle = Circle {
            center: point![1., 2., 3.],
            radius: vector![1., 0.],
        };

        assert_eq!(
            circle.point_model_to_curve(&point![2., 2., 3.]),
            point![0.],
        );
        assert_eq!(
            circle.point_model_to_curve(&point![1., 3., 3.]),
            point![FRAC_PI_2],
        );
        assert_eq!(
            circle.point_model_to_curve(&point![0., 2., 3.]),
            point![PI],
        );
        assert_eq!(
            circle.point_model_to_curve(&point![1., 1., 3.]),
            point![FRAC_PI_2 * 3.],
        );
    }

    #[test]
    fn number_of_vertices() {
        verify_result(50., 100., 3);
        verify_result(10., 100., 7);
        verify_result(1., 100., 23);

        fn verify_result(tolerance: f64, radius: f64, n: u64) {
            assert_eq!(n, Circle::number_of_vertices(tolerance, radius));

            assert!(calculate_error(radius, n) <= tolerance);
            if n > 3 {
                assert!(calculate_error(radius, n - 1) >= tolerance);
            }
        }

        fn calculate_error(radius: f64, n: u64) -> f64 {
            radius - radius * (PI / n as f64).cos()
        }
    }
}
