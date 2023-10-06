use fj_math::Point;

use crate::geometry::{CurveBoundaries, CurveBoundary};

use super::{CurveApproxPoints, CurveApproxSegment};

/// Partial approximation of a curve
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApprox {
    segments: CurveBoundaries<CurveApproxPoints>,
}

impl CurveApprox {
    /// Get the single segment that covers the provided boundary, if available
    pub fn into_single_segment(
        self,
        boundary: CurveBoundary<Point<1>>,
    ) -> Option<CurveApproxSegment> {
        self.segments
            .into_single_payload(boundary)
            .map(|points| CurveApproxSegment { boundary, points })
    }

    /// Reverse the approximation
    pub fn reverse(&mut self) {
        self.segments.reverse();
    }

    /// Reduce the approximation to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        self.segments.make_subset(boundary);
    }

    /// Merge the provided segment into the approximation
    pub fn merge(
        &mut self,
        new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        let (merged_boundary, merged_segment) = self
            .segments
            .merge(new_segment.boundary, new_segment.points);

        CurveApproxSegment {
            boundary: merged_boundary,
            points: merged_segment,
        }
    }
}

impl<const N: usize> From<[CurveApproxSegment; N]> for CurveApprox {
    fn from(segments: [CurveApproxSegment; N]) -> Self {
        Self {
            segments: CurveBoundaries {
                inner: segments
                    .into_iter()
                    .map(|segment| (segment.boundary, segment.points))
                    .collect(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::approx::{curve::CurveApproxSegment, ApproxPoint};

    use super::CurveApprox;

    #[test]
    fn reverse() {
        let mut approx = CurveApprox::from([
            CurveApproxSegment::from((
                [[0.1], [0.4]].into(),
                [
                    ApproxPoint::new([0.1], [0.1, 0.1, 0.1]),
                    ApproxPoint::new([0.4], [0.4, 0.4, 0.4]),
                ],
            )),
            CurveApproxSegment::from((
                [[0.6], [0.9]].into(),
                [
                    ApproxPoint::new([0.6], [0.6, 0.6, 0.6]),
                    ApproxPoint::new([0.9], [0.9, 0.9, 0.9]),
                ],
            )),
        ]);

        approx.reverse();

        assert_eq!(
            approx,
            CurveApprox::from([
                CurveApproxSegment::from((
                    [[0.9], [0.6]].into(),
                    [
                        ApproxPoint::new([0.9], [0.9, 0.9, 0.9]),
                        ApproxPoint::new([0.6], [0.6, 0.6, 0.6]),
                    ],
                )),
                CurveApproxSegment::from((
                    [[0.4], [0.1]].into(),
                    [
                        ApproxPoint::new([0.4], [0.4, 0.4, 0.4]),
                        ApproxPoint::new([0.1], [0.1, 0.1, 0.1]),
                    ],
                )),
            ])
        )
    }
}
