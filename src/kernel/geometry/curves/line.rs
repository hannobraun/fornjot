use crate::math::{Point, Vector};

/// A line, defined by two points
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    /// The origin point of the line
    ///
    /// The point on the plane that is the origin of the 1-dimensional curve
    /// coordinate system.
    pub origin: Point<3>,

    /// The direction of the line
    ///
    /// In addition to defining the direction of the line, the length of this
    /// vector defines the curve coordinate system: The point at `origin` +
    /// `dir` has curve coordinate `1.0`.
    pub dir: Vector<3>,
}
