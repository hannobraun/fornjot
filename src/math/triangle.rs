use super::Point;

/// A triangle
#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Point<3>,
    pub b: Point<3>,
    pub c: Point<3>,
}

impl Triangle {
    /// Convert the triangle to a Parry triangle
    pub fn to_parry(&self) -> parry3d_f64::shape::Triangle {
        self.vertices().into()
    }

    /// Access the triangle's vertices as an array
    pub fn vertices(&self) -> [Point<3>; 3] {
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
