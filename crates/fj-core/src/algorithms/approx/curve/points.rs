use fj_math::Point;

use crate::{algorithms::approx::ApproxPoint, geometry::CurveBoundary};

/// Points of a curve approximation
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApproxPoints {
    /// Points of a curve approximation
    pub inner: Vec<ApproxPoint<1>>,
}

impl CurveApproxPoints {
    /// Reverse the orientation of the approximation
    pub fn reverse(&mut self) -> &mut Self {
        self.inner.reverse();
        self
    }

    /// Reduce the approximation to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        self.inner
            .retain(|point| boundary.contains(point.local_form));
    }
}
