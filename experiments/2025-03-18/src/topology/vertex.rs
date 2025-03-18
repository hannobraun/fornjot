use crate::math::Point;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point<3>,
}

impl Vertex {
    pub fn new(point: impl Into<Point<3>>) -> Self {
        let point = point.into();
        Self { point }
    }
}
