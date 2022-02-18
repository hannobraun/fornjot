use super::Point;

/// A line segment, defined by its two end points
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment<const D: usize> {
    pub a: Point<D>,
    pub b: Point<D>,
}

impl Segment<3> {
    /// Convert the segment to a Parry segment
    pub fn to_parry(&self) -> parry3d_f64::shape::Segment {
        [self.a, self.b].into()
    }
}

impl From<[Point<2>; 2]> for Segment<2> {
    fn from(points: [Point<2>; 2]) -> Self {
        Self {
            a: points[0],
            b: points[1],
        }
    }
}

impl From<[Point<3>; 2]> for Segment<3> {
    fn from(points: [Point<3>; 2]) -> Self {
        Self {
            a: points[0],
            b: points[1],
        }
    }
}
