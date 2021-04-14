use parry2d::shape::Triangle;

use super::{Pnt2, Seg2};

pub struct Tri2 {
    pub a: Pnt2,
    pub b: Pnt2,
    pub c: Pnt2,
}

impl Tri2 {
    pub fn vertices(&self) -> [Pnt2; 3] {
        [self.a, self.b, self.c]
    }

    pub fn edges(&self) -> [Seg2; 3] {
        [
            Seg2::new(self.a, self.b),
            Seg2::new(self.b, self.c),
            Seg2::new(self.c, self.a),
        ]
    }
}

impl From<Triangle> for Tri2 {
    fn from(triangle: Triangle) -> Self {
        Tri2 {
            a: triangle.a.into(),
            b: triangle.b.into(),
            c: triangle.c.into(),
        }
    }
}
