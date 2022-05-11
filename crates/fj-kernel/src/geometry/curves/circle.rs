use fj_math::{Point, Scalar, Transform, Vector};

/// A circle
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Circle {
    /// The center point of the circle
    pub center: Point<3>,

    /// A vector from the center to the starting point of the circle
    ///
    /// The length of this vector defines the circle radius. Please also refer
    /// to the documentation of `b`.
    pub a: Vector<3>,

    /// A second vector that defines the plane of the circle
    ///
    /// The vector must be of equal length to `a` (the circle radius) and must
    /// be perpendicular to it. Code working with circles might assume that
    /// these conditions are met.
    pub b: Vector<3>,
}

impl Circle {
    /// Access the origin of the curve's coordinate system
    pub fn origin(&self) -> Point<3> {
        self.center
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.b = -self.b;
        self
    }

    /// Create a new instance that is transformed by `transform`
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        Self {
            center: transform.transform_point(&self.center),
            a: transform.transform_vector(&self.a),
            b: transform.transform_vector(&self.b),
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
    pub fn point_to_circle_coords(&self, point: &Point<3>) -> Point<1> {
        let v = point - self.center;
        let atan = Scalar::atan2(v.y, v.x);
        let coord = if atan >= Scalar::ZERO {
            atan
        } else {
            atan + Scalar::PI * 2.
        };
        Point::from([coord])
    }

    /// Convert a point on the curve into model coordinates
    pub fn point_curve_to_model(&self, point: &Point<1>) -> Point<3> {
        self.center + self.vector_curve_to_model(&point.coords)
    }

    /// Convert a vector on the curve into model coordinates
    pub fn vector_curve_to_model(&self, vector: &Vector<1>) -> Vector<3> {
        let angle = vector.t;
        let (sin, cos) = angle.sin_cos();

        self.a * cos + self.b * sin
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use fj_math::{Point, Vector};

    use super::Circle;

    #[test]
    fn point_model_to_curve() {
        let circle = Circle {
            center: Point::from([1., 2., 3.]),
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., 1., 0.]),
        };

        assert_eq!(
            circle.point_to_circle_coords(&Point::from([2., 2., 3.])),
            Point::from([0.]),
        );
        assert_eq!(
            circle.point_to_circle_coords(&Point::from([1., 3., 3.])),
            Point::from([FRAC_PI_2]),
        );
        assert_eq!(
            circle.point_to_circle_coords(&Point::from([0., 2., 3.])),
            Point::from([PI]),
        );
        assert_eq!(
            circle.point_to_circle_coords(&Point::from([1., 1., 3.])),
            Point::from([FRAC_PI_2 * 3.]),
        );
    }
}
