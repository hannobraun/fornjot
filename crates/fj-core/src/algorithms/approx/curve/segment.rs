use std::cmp;

use fj_math::Point;

use crate::{algorithms::approx::ApproxPoint, geometry::CurveBoundary};

use super::points::CurveApproxPoints;

/// A segment of a curve approximation
///
/// A curve is potentially infinite (at least its local coordinate space is
/// infinite, even if the curve itself isn't; a circle is an example of that).
/// This means a curve can only be approximated locally, at a number of
/// segments. This struct represents on such segment.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CurveApproxSegment {
    /// The boundary within which this segment approximates the curve
    pub boundary: CurveBoundary<Point<1>>,

    /// The points that approximate the curve segment
    pub points: CurveApproxPoints,
}

impl CurveApproxSegment {
    /// Indicate whether the segment is empty
    pub fn is_empty(&self) -> bool {
        let is_empty = self.boundary.is_empty();

        if is_empty {
            assert!(
                self.points.is_empty(),
                "Empty approximation still has points"
            );
        }

        is_empty
    }

    /// Indicate whether the segment is normalized
    pub fn is_normalized(&self) -> bool {
        self.boundary.is_normalized()
    }

    /// Reverse the orientation of the approximation
    pub fn reverse(&mut self) -> &mut Self {
        self.boundary = self.boundary.reverse();
        self.points.reverse();
        self
    }

    /// Normalize the segment
    ///
    /// Puts the points of the approximation in a defined order. This can be
    /// used to compare segments while disregarding their direction.
    pub fn normalize(&mut self) -> &mut Self {
        if !self.is_normalized() {
            self.boundary = self.boundary.normalize();
            self.points.inner.reverse();
        }

        self
    }

    /// Reduce the approximation to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        assert!(
            self.is_normalized(),
            "Expected normalized segment for making subset."
        );
        assert!(
            boundary.is_normalized(),
            "Expected subset to be defined by normalized boundary."
        );

        self.boundary = self.boundary.subset(boundary);
        self.points.make_subset(boundary);
    }
}

impl Ord for CurveApproxSegment {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let [a_start, a_end] = self.boundary.inner;
        let [b_start, b_end] = other.boundary.inner;

        let by_start = a_start.cmp(&b_start);
        if by_start.is_ne() {
            return by_start;
        }

        a_end.cmp(&b_end)
    }
}

impl PartialOrd for CurveApproxSegment {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const D: usize> From<(CurveBoundary<Point<1>>, [ApproxPoint<1>; D])>
    for CurveApproxSegment
{
    fn from(
        (boundary, points): (CurveBoundary<Point<1>>, [ApproxPoint<1>; D]),
    ) -> Self {
        Self {
            boundary,
            points: CurveApproxPoints {
                inner: points.into_iter().collect(),
            },
        }
    }
}
