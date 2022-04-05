use super::{Point, Scalar};

/// A triangle
///
/// The dimensionality of the triangle is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Triangle<const D: usize> {
    points: [Point<D>; 3],
    color: [u8; 4],
}

impl<const D: usize> Triangle<D> {
    /// Construct a triangle from three points
    ///
    /// # Panics
    ///
    /// Panics, if the points don't form a triangle.
    pub fn from_points(points: [Point<D>; 3]) -> Self {
        let area = {
            let [a, b, c] = points.map(Point::to_xyz);
            (b - a).cross(&(c - a)).magnitude()
        };

        // A triangle is not valid if it doesn't span any area
        if area != Scalar::from(0.0) {
            Self {
                points,
                color: [255, 0, 0, 255],
            }
        } else {
            panic!("Invalid Triangle specified");
        }
    }

    /// Access the triangle's points
    pub fn points(&self) -> [Point<D>; 3] {
        self.points
    }

    /// Return the specified color of the triangle in RGBA
    pub fn color(&self) -> [u8; 4] {
        self.color
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

    /// Set a new color for the particular triangle
    pub fn set_color(&mut self, color: [u8; 4]) {
        self.color = color;
    }
}

impl Triangle<3> {
    /// Convert the triangle to a Parry triangle
    pub fn to_parry(self) -> parry3d_f64::shape::Triangle {
        self.points().map(|vertex| vertex.to_na()).into()
    }
}

impl<const D: usize> From<[Point<D>; 3]> for Triangle<D> {
    fn from(points: [Point<D>; 3]) -> Self {
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

    #[test]
    fn triangle_default_color() {
        let a = Point::from([0.0, 0.0]);
        let b = Point::from([1.0, 1.0]);
        let c = Point::from([1.0, 2.0]);
        let triangle = Triangle::from([a, b, c]);
        assert_eq!(triangle.color(), [255, 0, 0, 255]);
    }

    #[test]
    fn triangle_set_color() {
        let a = Point::from([0.0, 0.0]);
        let b = Point::from([1.0, 1.0]);
        let c = Point::from([1.0, 2.0]);
        let mut triangle = Triangle::from([a, b, c]);
        triangle.set_color([1, 2, 3, 4]);
        assert_eq!(triangle.color(), [1, 2, 3, 4]);
    }
}
