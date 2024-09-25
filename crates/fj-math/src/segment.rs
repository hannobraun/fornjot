use std::fmt;

use crate::Scalar;

use super::Point;

/// A line segment, defined by its two end points
///
/// The dimensionality of the segment is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Eq, Default, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct LineSegment<const D: usize> {
    /// # The end points of the line segment
    pub points: [Point<D>; 2],
}

impl<const D: usize> LineSegment<D> {
    /// Compute the center point of the segment
    pub fn center(&self) -> Point<D> {
        let [a, b] = self.points;
        a + (b - a) / Scalar::TWO
    }

    /// Create a segment with the same points in the opposite order
    pub fn reverse(mut self) -> Self {
        self.points.reverse();
        self
    }
}

impl LineSegment<2> {
    /// Convert the 2-dimensional segment to a Parry segment
    pub fn to_parry(self) -> parry2d_f64::shape::Segment {
        self.points.map(|point| point.to_na()).into()
    }
}

impl LineSegment<3> {
    /// Convert the 3-dimensional segment to a Parry segment
    pub fn to_parry(self) -> parry3d_f64::shape::Segment {
        self.points.map(|point| point.to_na()).into()
    }
}

impl<P, const D: usize> From<[P; 2]> for LineSegment<D>
where
    P: Into<Point<D>>,
{
    fn from(points: [P; 2]) -> Self {
        Self {
            points: points.map(Into::into),
        }
    }
}

impl<const D: usize> fmt::Debug for LineSegment<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?} -> {:?}]", self.points[0], self.points[1])
    }
}
