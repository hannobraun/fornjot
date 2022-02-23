use std::fmt;

use super::Point;

/// A line segment, defined by its two end points
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Segment<const D: usize> {
    a: Point<D>,
    b: Point<D>,
}

impl<const D: usize> Segment<D> {
    /// Access the points of the segment
    pub fn points(&self) -> [Point<D>; 2] {
        [self.a, self.b]
    }
}

impl Segment<2> {
    /// Convert the 2-dimensional segment to a Parry segment
    pub fn to_parry(&self) -> parry2d_f64::shape::Segment {
        [self.a.to_na(), self.b.to_na()].into()
    }
}

impl Segment<3> {
    /// Convert the 3-dimensional segment to a Parry segment
    pub fn to_parry(&self) -> parry3d_f64::shape::Segment {
        [self.a.to_na(), self.b.to_na()].into()
    }
}

impl<const D: usize> From<[Point<D>; 2]> for Segment<D> {
    fn from(points: [Point<D>; 2]) -> Self {
        let [a, b] = points;

        if a == b {
            panic!("Invalid segment; both points are identical {a:?}");
        }

        Self {
            a: points[0],
            b: points[1],
        }
    }
}

impl<const D: usize> fmt::Debug for Segment<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?} -> {:?}]", self.a, self.b)
    }
}
