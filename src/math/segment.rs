use super::Point;

/// A line segment, defined by its two end points
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    pub a: Point<3>,
    pub b: Point<3>,
}

impl Segment {
    /// Convert the segment into a Parry segment
    pub fn to_parry(&self) -> parry3d_f64::shape::Segment {
        [self.a, self.b].into()
    }
}

impl From<[Point<3>; 2]> for Segment {
    fn from(points: [Point<3>; 2]) -> Self {
        Self {
            a: points[0],
            b: points[1],
        }
    }
}
