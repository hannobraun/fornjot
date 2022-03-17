use std::fmt;

use super::Point;

/// A line segment, defined by its two end points
///
/// The dimensionality of the segment is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Segment<const D: usize> {
    points: [Point<D>; 2],
}

impl<const D: usize> Segment<D> {
    /// Construct a segment from two points
    ///
    /// # Panics
    ///
    /// Panics, if the points are coincident.
    pub fn from_points(points: [Point<D>; 2]) -> Self {
        let [a, b] = points;

        assert!(a != b, "Invalid segment; both points are identical {a:?}");

        Self { points }
    }

    /// Access the points of the segment
    pub fn points(&self) -> [Point<D>; 2] {
        self.points
    }
}

impl Segment<2> {
    /// Convert the 2-dimensional segment to a Parry segment
    pub fn to_parry(self) -> parry2d_f64::shape::Segment {
        self.points.map(|point| point.to_na()).into()
    }
}

impl Segment<3> {
    /// Convert the 3-dimensional segment to a Parry segment
    pub fn to_parry(self) -> parry3d_f64::shape::Segment {
        self.points.map(|point| point.to_na()).into()
    }
}

impl<const D: usize> From<[Point<D>; 2]> for Segment<D> {
    fn from(points: [Point<D>; 2]) -> Self {
        Self::from_points(points)
    }
}

impl<const D: usize> fmt::Debug for Segment<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?} -> {:?}]", self.points[0], self.points[1])
    }
}
