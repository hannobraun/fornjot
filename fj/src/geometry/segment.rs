use parry2d::shape::Segment;

use super::point::Pnt2;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Seg2 {
    pub a: Pnt2,
    pub b: Pnt2,
}

impl Seg2 {
    pub fn new(a: impl Into<Pnt2>, b: impl Into<Pnt2>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn normalize(&self) -> Self {
        if self.a <= self.b {
            *self
        } else {
            Self {
                a: self.b,
                b: self.a,
            }
        }
    }
}

impl From<Segment> for Seg2 {
    fn from(segment: Segment) -> Self {
        Self {
            a: segment.a.into(),
            b: segment.b.into(),
        }
    }
}

impl From<&Segment> for Seg2 {
    fn from(segment: &Segment) -> Self {
        Self {
            a: segment.a.into(),
            b: segment.b.into(),
        }
    }
}

impl From<Seg2> for Segment {
    fn from(segment: Seg2) -> Self {
        Self {
            a: segment.a.into(),
            b: segment.b.into(),
        }
    }
}

impl From<&Seg2> for Segment {
    fn from(segment: &Seg2) -> Self {
        Self {
            a: segment.a.into(),
            b: segment.b.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Pnt2;

    use super::Seg2;

    #[test]
    fn it_should_normalize_a_segment() {
        let a = Seg2 {
            a: Pnt2::from_f32s(0.0, 0.0),
            b: Pnt2::from_f32s(1.0, 1.0),
        };
        let b = Seg2 { a: a.b, b: a.a };

        assert_eq!(a.normalize(), a);
        assert_eq!(b.normalize(), a);
    }
}
