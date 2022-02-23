use super::Point;

/// A triangle
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Triangle {
    a: Point<3>,
    b: Point<3>,
    c: Point<3>,
}

impl Triangle {
    /// Convert the triangle to a Parry triangle
    pub fn to_parry(&self) -> parry3d_f64::shape::Triangle {
        self.points().map(|vertex| vertex.to_na()).into()
    }

    /// Access the triangle's vertices as an array
    pub fn points(&self) -> [Point<3>; 3] {
        [self.a, self.b, self.c]
    }
}

impl From<[Point<3>; 3]> for Triangle {
    fn from(points: [Point<3>; 3]) -> Self {
        Self {
            a: points[0],
            b: points[1],
            c: points[2],
        }
    }
}
