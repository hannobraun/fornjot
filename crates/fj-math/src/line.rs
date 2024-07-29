use crate::{Point, Scalar, Triangle, Vector};

/// An n-dimensional line, defined by an origin and a direction
///
/// The dimensionality of the line is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Line<const D: usize> {
    origin: Point<D>,
    direction: Vector<D>,
}

impl<const D: usize> Line<D> {
    /// Create a line from a point and a vector
    ///
    /// # Panics
    ///
    /// Panics, if `direction` has a length of zero.
    pub fn from_origin_and_direction(
        origin: Point<D>,
        direction: Vector<D>,
    ) -> Self {
        assert!(
            direction.magnitude() != Scalar::ZERO,
            "Can't construct `Line`. Direction is zero: {direction:?}"
        );

        Self { origin, direction }
    }

    /// Create a line from two points
    ///
    /// Also returns the lines coordinates of the provided points on the new
    /// line.
    ///
    /// # Panics
    ///
    /// Panics, if the points are coincident.
    pub fn from_points(
        points: [impl Into<Point<D>>; 2],
    ) -> (Self, [Point<1>; 2]) {
        let [a, b] = points.map(Into::into);

        let line = Self::from_origin_and_direction(a, b - a);
        let coords = [[0.], [1.]].map(Point::from);

        (line, coords)
    }

    /// Create a line from two points that include line coordinates
    ///
    /// # Panics
    ///
    /// Panics, if the points are coincident.
    pub fn from_points_with_line_coords(
        points: [(impl Into<Point<1>>, impl Into<Point<D>>); 2],
    ) -> Self {
        let [(a_line, a_global), (b_line, b_global)] =
            points.map(|(point_line, point_global)| {
                (point_line.into(), point_global.into())
            });

        let direction = (b_global - a_global) / (b_line - a_line).t;
        let origin = a_global + direction * -a_line.t;

        Self::from_origin_and_direction(origin, direction)
    }

    /// Access the origin of the line
    ///
    /// The origin is a point on the line which, together with the `direction`
    /// field, defines the line fully. The origin also defines the origin of the
    /// line's 1-dimensional coordinate system.
    pub fn origin(&self) -> Point<D> {
        self.origin
    }

    /// Access the direction of the line
    ///
    /// The length of this vector defines the unit of the line's curve
    /// coordinate system. The coordinate `1.` is always where the direction
    /// vector points, from `origin`.
    pub fn direction(&self) -> Vector<D> {
        self.direction
    }

    /// Determine if this line is coincident with another line
    ///
    /// # Implementation Note
    ///
    /// This method only returns `true`, if the lines are precisely coincident.
    /// This will probably not be enough going forward, but it'll do for now.
    pub fn is_coincident_with(&self, other: &Self) -> bool {
        let other_origin_is_not_on_self = {
            let a = other.origin;
            let b = self.origin;
            let c = self.origin + self.direction;

            // The triangle is valid only, if the three points are not on the
            // same line.
            Triangle::from_points([a, b, c]).is_valid()
        };

        if other_origin_is_not_on_self {
            return false;
        }

        let d1 = self.direction.normalize();
        let d2 = other.direction.normalize();

        d1 == d2 || d1 == -d2
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.origin += self.direction;
        self.direction = -self.direction;
        self
    }

    /// Convert a `D`-dimensional point to line coordinates
    ///
    /// Projects the point onto the line before the conversion. This is done to
    /// make this method robust against floating point accuracy issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the line, intentional or not, will never result in an
    /// error.
    pub fn point_to_line_coords(&self, point: impl Into<Point<D>>) -> Point<1> {
        Point {
            coords: self.vector_to_line_coords(point.into() - self.origin),
        }
    }

    /// Convert a `D`-dimensional vector to line coordinates
    pub fn vector_to_line_coords(
        &self,
        vector: impl Into<Vector<D>>,
    ) -> Vector<1> {
        let t = vector.into().scalar_projection_onto(&self.direction)
            / self.direction.magnitude();
        Vector::from([t])
    }

    /// Convert a point in line coordinates into a `D`-dimensional point
    pub fn point_from_line_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<D> {
        self.origin + self.vector_from_line_coords(point.into().coords)
    }

    /// Convert a vector in line coordinates into a `D`-dimensional vector
    pub fn vector_from_line_coords(
        &self,
        vector: impl Into<Vector<1>>,
    ) -> Vector<D> {
        self.direction * vector.into().t
    }
}

impl<const D: usize> approx::AbsDiffEq for Line<D> {
    type Epsilon = <Scalar as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        Scalar::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.direction.abs_diff_eq(&other.direction, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{Point, Scalar, Vector};

    use super::Line;

    #[test]
    fn from_points_with_line_coords() {
        let line = Line::from_points_with_line_coords([
            ([0.], [0., 0.]),
            ([1.], [1., 0.]),
        ]);
        assert_eq!(line.origin(), Point::from([0., 0.]));
        assert_eq!(line.direction(), Vector::from([1., 0.]));

        let line = Line::from_points_with_line_coords([
            ([1.], [0., 1.]),
            ([0.], [1., 1.]),
        ]);
        assert_eq!(line.origin(), Point::from([1., 1.]));
        assert_eq!(line.direction(), Vector::from([-1., 0.]));

        let line = Line::from_points_with_line_coords([
            ([-1.], [0., 2.]),
            ([0.], [1., 2.]),
        ]);
        assert_eq!(line.origin(), Point::from([1., 2.]));
        assert_eq!(line.direction(), Vector::from([1., 0.]));
    }

    #[test]
    fn is_coincident_with() {
        let (line, _) = Line::from_points([[0., 0.], [1., 0.]]);

        let (a, _) = Line::from_points([[0., 0.], [1., 0.]]);
        let (b, _) = Line::from_points([[0., 0.], [-1., 0.]]);
        let (c, _) = Line::from_points([[0., 1.], [1., 1.]]);

        assert!(line.is_coincident_with(&a));
        assert!(line.is_coincident_with(&b));
        assert!(!line.is_coincident_with(&c));
    }

    #[test]
    fn convert_point_to_line_coords() {
        let line = Line {
            origin: Point::from([1., 2., 3.]),
            direction: Vector::from([2., 3., 5.]),
        };

        verify(line, -1.);
        verify(line, 0.);
        verify(line, 1.);
        verify(line, 2.);

        fn verify(line: Line<3>, t: f64) {
            let point = line.point_from_line_coords([t]);
            let t_result = line.point_to_line_coords(point);

            assert_abs_diff_eq!(
                Point::from([t]),
                t_result,
                epsilon = Scalar::from(1e-8)
            );
        }
    }
}
