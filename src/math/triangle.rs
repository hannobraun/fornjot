use super::Point;

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
        Self {
            a: points[0],
            b: points[1],
            c: points[2],
        }
    }
}
