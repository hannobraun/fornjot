use crate::math::Point;

/// A line, defined by two points
#[derive(Clone, Debug)]
pub struct Line {
    /// One point defining the line
    pub a: Point<3>,

    /// The other point defining the line
    pub b: Point<3>,
}
