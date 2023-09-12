use std::cmp;

use fj_math::Point;

use crate::{algorithms::approx::ApproxPoint, geometry::CurveBoundary};

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
    pub points: Vec<ApproxPoint<1>>,
}

impl CurveApproxSegment {
    /// Indicate whether the segment is empty
    pub fn is_empty(&self) -> bool {
        let is_empty = {
            let [min, max] = self.boundary.inner;
            min >= max
        };

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

    /// Indicate whether this segment overlaps another
    ///
    /// Segments that touch (i.e. their closest boundary is equal) count as
    /// overlapping.
    pub fn overlaps(&self, other: &Self) -> bool {
        let [a_low, a_high] = self.boundary.normalize().inner;
        let [b_low, b_high] = other.boundary.normalize().inner;

        a_low <= b_high && a_high >= b_low
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
            self.points.reverse();
        }

        self
    }

    /// Reduce the approximation to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        assert!(
            self.boundary.is_normalized(),
            "Expected normalized segment for making subset."
        );
        assert!(
            boundary.is_normalized(),
            "Expected subset to be defined by normalized boundary."
        );

        let [min, max] = boundary.inner;

        self.boundary.inner = {
            let [self_min, self_max] = self.boundary.inner;

            let min = cmp::max(self_min, min);
            let max = cmp::min(self_max, max);

            [min, max]
        };

        self.points
            .retain(|point| point.local_form > min && point.local_form < max);
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

#[cfg(test)]
mod tests {
    use crate::algorithms::approx::curve::CurveApproxSegment;

    #[test]
    fn overlaps() {
        assert!(overlap([0., 2.], [1., 3.])); // regular overlap
        assert!(overlap([0., 1.], [1., 2.])); // just touching
        assert!(overlap([2., 0.], [3., 1.])); // not normalized
        assert!(overlap([1., 3.], [0., 2.])); // lower boundary comes second

        assert!(!overlap([0., 1.], [2., 3.])); // regular non-overlap
        assert!(!overlap([2., 3.], [0., 1.])); // lower boundary comes second

        fn overlap(a: [f64; 2], b: [f64; 2]) -> bool {
            let a = CurveApproxSegment {
                boundary: a.map(|coord| [coord]).into(),
                points: Vec::new(),
            };
            let b = CurveApproxSegment {
                boundary: b.map(|coord| [coord]).into(),
                points: Vec::new(),
            };

            a.overlaps(&b)
        }
    }
}
