use crate::math::Point;

/// A line, defined by two points
#[derive(Clone, Debug)]
pub struct Line {
    /// The origin point of the line
    ///
    /// The point on the plane that is the origin of the 1-dimensional curve
    /// coordinate system.
    pub origin: Point<3>,

    /// The other point defining the line
    pub b: Point<3>,
}
