use super::{Point, Scalar};

/// A triangle
///
/// The dimensionality of the triangle is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Triangle<const D: usize> {
    points: [Point<D>; 3],
}

impl<const D: usize> Triangle<D> {
    /// Construct a triangle from three points
    ///
    /// # Panics
    ///
    /// Panics, if the points don't form a triangle.
    pub fn from_points(points: [impl Into<Point<D>>; 3]) -> Self {
        let points = points.map(Into::into);

        let area = {
            let [a, b, c] = points.map(Point::to_xyz);
            (b - a).cross(&(c - a)).magnitude()
        };

        // A triangle is not valid if it doesn't span any area
        if area != Scalar::from(0.0) {
            Self { points }
        } else {
            panic!("Invalid Triangle specified");
        }
    }

    /// Access the triangle's points
    pub fn points(&self) -> [Point<D>; 3] {
        self.points
    }

    /// Normalize the triangle
    ///
    /// Returns a new `Triangle` instance with the same points, but the points
    /// ordered such that they are ordered according to their `Ord`/`PartialOrd`
    /// implementation.
    ///
    /// This is useful for comparing triangles, where the order of points is not
    /// important.
    pub fn normalize(mut self) -> Self {
        self.points.sort();
        self
    }
}

impl Triangle<3> {
    /// Convert the triangle to a Parry triangle
    pub fn to_parry(self) -> parry3d_f64::shape::Triangle {
        self.points().map(|vertex| vertex.to_na()).into()
    }
}

impl<P, const D: usize> From<[P; 3]> for Triangle<D>
where
    P: Into<Point<D>>,
{
    fn from(points: [P; 3]) -> Self {
        let points = points.map(Into::into);
        Self::from_points(points)
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    use super::Triangle;

    #[test]
    fn valid_triangle_2d() {
        let a = Point::from([0.0, 0.0]);
        let b = Point::from([1.0, 1.0]);
        let c = Point::from([1.0, 2.0]);
        let _triangle = Triangle::from([a, b, c]);
    }

    #[test]
    fn valid_triangle_3d() {
        let a = Point::from([0.0, 0.0, 0.0]);
        let b = Point::from([1.0, 1.0, 0.0]);
        let c = Point::from([1.0, 2.0, 0.0]);
        let _triangle = Triangle::from([a, b, c]);
    }

    #[test]
    #[should_panic]
    fn invalid_triangle_2d() {
        let a = Point::from([0.0, 0.0]);
        let b = Point::from([1.0, 1.0]);
        let c = Point::from([2.0, 2.0]);
        let _triangle = Triangle::from([a, b, c]);
    }

    #[test]
    #[should_panic]
    fn invalid_triangle_3d() {
        let a = Point::from([0.0, 0.0, 0.0]);
        let b = Point::from([1.0, 1.0, 1.0]);
        let c = Point::from([2.0, 2.0, 2.0]);
        let _triangle = Triangle::from([a, b, c]);
    }
}
