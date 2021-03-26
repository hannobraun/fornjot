use parry2d::shape::Segment;

use super::point::Pnt2;

pub struct Seg2 {
    pub a: Pnt2,
    pub b: Pnt2,
}

impl From<Seg2> for Segment {
    fn from(segment: Seg2) -> Self {
        Self {
            a: segment.a.into(),
            b: segment.b.into(),
        }
    }
}
