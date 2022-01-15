use approx::AbsDiffEq;

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

impl AbsDiffEq for Line {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.dir.abs_diff_eq(&other.dir, epsilon)
    }
}
