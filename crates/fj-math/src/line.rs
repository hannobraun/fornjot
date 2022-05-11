use crate::{Point, Vector};

/// An n-dimensional line, defined by an origin and a direction
///
/// The dimensionality of the line is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Line<const D: usize> {
    /// The origin of the line
    ///
    /// The origin is a point on the line which, together with the `direction`
    /// field, defines the line fully. The origin also defines the origin of the
    /// line's 1-dimensional coordinate system.
    pub origin: Point<D>,

    /// The direction of the line
    ///
    /// The length of this vector defines the unit of the line's curve
    /// coordinate system. The coordinate `1.` is always were the direction
    /// vector points, from `origin`.
    pub direction: Vector<D>,
}

impl<const D: usize> Line<D> {
    /// Create a line from two points
    pub fn from_points([a, b]: [Point<D>; 2]) -> Self {
        Self {
            origin: a,
            direction: b - a,
        }
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(mut self) -> Self {
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
    pub fn point_to_line_coords(&self, point: &Point<D>) -> Point<1> {
        Point {
            coords: self.vector_to_line_coords(&(point - self.origin)),
        }
    }

    /// Convert a `D`-dimensional vector to line coordinates
    pub fn vector_to_line_coords(&self, vector: &Vector<D>) -> Vector<1> {
        let t = vector.scalar_projection_onto(&self.direction)
            / self.direction.magnitude();
        Vector::from([t])
    }

    /// Convert a point in line coordinates into a `D`-dimensional point
    pub fn point_from_line_coords(&self, point: &Point<1>) -> Point<D> {
        self.origin + self.vector_from_line_coords(&point.coords)
    }

    /// Convert a vector in line coordinates into a `D`-dimensional vector
    pub fn vector_from_line_coords(&self, vector: &Vector<1>) -> Vector<D> {
        self.direction * vector.t
    }
}

impl<const D: usize> approx::AbsDiffEq for Line<D> {
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
    use crate::{Point, Vector};

    use super::Line;

    #[test]
    fn convert_point_to_line_coords() {
        let line = Line {
            origin: Point::from([1., 0., 0.]),
            direction: Vector::from([2., 0., 0.]),
        };

        verify(line, -1.);
        verify(line, 0.);
        verify(line, 1.);
        verify(line, 2.);

        fn verify(line: Line<3>, t: f64) {
            let point = line.point_from_line_coords(&Point::from([t]));
            let t_result = line.point_to_line_coords(&point);

            assert_eq!(Point::from([t]), t_result);
        }
    }
}
