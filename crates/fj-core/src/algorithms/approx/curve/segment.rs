use crate::algorithms::approx::ApproxPoint;

/// A segment of a curve approximation
///
/// A curve is potentially infinite (at least its local coordinate space is
/// infinite, even if the curve itself isn't; a circle is an example of that).
/// This means a curve can only be approximated locally, at a number of
/// segments. This struct represents on such segment.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApproxSegment {
    /// The points that approximate the curve segment
    pub points: Vec<ApproxPoint<1>>,
}

impl CurveApproxSegment {
    /// Reverse the orientation of the approximation
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.points.reverse();
        self
    }
}
