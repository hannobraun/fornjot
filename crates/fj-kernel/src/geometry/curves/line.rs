use fj_math::{Point, Transform, Vector};

/// A line, defined by a point and a vector
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Line {
    /// The origin of the line's coordinate system
    pub origin: Point<3>,

    /// The direction of the line
    ///
    /// The length of this vector defines the unit of the line's curve
    /// coordinate system. The coordinate `1.` is always were the direction
    /// vector points, from `origin`.
    pub direction: Vector<3>,
}

impl Line {
    /// Create a line from two points
    pub fn from_points([a, b]: [Point<3>; 2]) -> Self {
        Self {
            origin: a,
            direction: b - a,
        }
    }

    /// Access the origin of the curve's coordinate system
    pub fn origin(&self) -> Point<3> {
        self.origin
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.direction = -self.direction;
        self
    }

    /// Create a new instance that is transformed by `transform`
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        Self {
            origin: transform.transform_point(&self.origin),
            direction: transform.transform_vector(&self.direction),
        }
    }

    /// Convert a point in model coordinates to curve coordinates
    ///
    /// Projects the point onto the line before computing curve coordinate. This
    /// is done to make this method robust against floating point accuracy
    /// issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the line, intentional or not, will never result in an
    /// error.
    pub fn point_model_to_curve(&self, point: &Point<3>) -> Point<1> {
        // scalar projection
        let t = (point - self.origin).dot(&self.direction.normalize())
            / self.direction.magnitude();

        Point::from([t])
    }

    /// Convert a point on the curve into model coordinates
    pub fn point_curve_to_model(&self, point: &Point<1>) -> Point<3> {
        self.origin + self.vector_curve_to_model(&point.coords)
    }

    /// Convert a vector on the curve into model coordinates
    pub fn vector_curve_to_model(&self, vector: &Vector<1>) -> Vector<3> {
        self.direction * vector.t
    }
}

impl approx::AbsDiffEq for Line {
    type Epsilon = <f64 as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.direction.abs_diff_eq(&other.direction, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use fj_math::{Point, Scalar, Transform, Vector};

    use super::Line;

    #[test]
    fn transform() {
        let line = Line {
            origin: Point::from([1., 0., 0.]),
            direction: Vector::from([0., 1., 0.]),
        };

        let transform = Transform::translation([1., 2., 3.])
            * Transform::rotation(Vector::unit_z() * (Scalar::PI / 2.));
        let line = line.transform(&transform);

        assert_abs_diff_eq!(
            line,
            Line {
                origin: Point::from([1., 3., 3.]),
                direction: Vector::from([-1., 0., 0.]),
            },
            epsilon = 1e-8,
        );
    }

    #[test]
    fn point_model_to_curve() {
        let line = Line {
            origin: Point::from([1., 0., 0.]),
            direction: Vector::from([2., 0., 0.]),
        };

        verify(line, -1.);
        verify(line, 0.);
        verify(line, 1.);
        verify(line, 2.);

        fn verify(line: Line, t: f64) {
            let point = line.point_curve_to_model(&Point::from([t]));
            let t_result = line.point_model_to_curve(&point);

            assert_eq!(Point::from([t]), t_result);
        }
    }
}
