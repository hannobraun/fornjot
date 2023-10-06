use fj_math::Point;

use crate::{
    algorithms::approx::ApproxPoint,
    geometry::{CurveBoundariesPayload, CurveBoundary},
};

/// Points of a curve approximation
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApproxPoints {
    /// Points of a curve approximation
    pub inner: Vec<ApproxPoint<1>>,
}

impl CurveApproxPoints {
    /// Indicate whether there are any points
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Reverse the orientation of the approximation
    pub fn reverse(&mut self) {
        self.inner.reverse();
    }

    /// Reduce the approximation to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        self.inner
            .retain(|point| boundary.contains(point.local_form));
    }

    /// Merge the provided points
    ///
    /// If there is a true overlap between these points and the other points
    /// then the overlapping part is taken from the other points.
    pub fn merge(
        &mut self,
        other: &Self,
        other_boundary: CurveBoundary<Point<1>>,
    ) {
        self.inner.retain(|point| {
            // Only retain points that don't overlap with the other points, or
            // we might end up with duplicates.
            !other_boundary.contains(point.local_form)
        });
        self.inner.extend(&other.inner);
        self.inner.sort();
    }
}

impl CurveBoundariesPayload for CurveApproxPoints {
    fn reverse(&mut self) {
        self.reverse();
    }

    fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        self.make_subset(boundary)
    }
}
