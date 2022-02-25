use super::{Point, Scalar};

/// A triangle
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Triangle<const D: usize> {
    a: Point<D>,
    b: Point<D>,
    c: Point<D>,
}

impl<const D: usize> Triangle<D> {
    /// Access the triangle's points
    pub fn points(&self) -> [Point<D>; 3] {
        [self.a, self.b, self.c]
    }
}

impl Triangle<3> {
    /// Convert the triangle to a Parry triangle
    pub fn to_parry(&self) -> parry3d_f64::shape::Triangle {
        self.points().map(|vertex| vertex.to_na()).into()
    }
}

impl<const D: usize> From<[Point<D>; 3]> for Triangle<D> {
    fn from(points: [Point<D>; 3]) -> Self {
        let a = points[0];
        let b = points[1];
        let c = points[2];
        // A triangle is not valid if it doesn't span any area
        if (b - a).cross(&(c - a)).magnitude() != Scalar::from(0.0) {
            Self { a, b, c }
        } else {
            panic!("Invalid Triangle specified");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Triangle;
    use crate::math::Point;

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
