use std::collections::VecDeque;

use fj_math::Point;

use crate::geometry::CurveBoundary;

use super::CurveApproxSegment;

/// Partial approximation of a curve
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApprox {
    /// The approximated segments that are part of this approximation
    pub segments: Vec<CurveApproxSegment>,
}

impl CurveApprox {
    /// Reverse the approximation
    pub fn reverse(&mut self) -> &mut Self {
        self.segments.reverse();

        for segment in &mut self.segments {
            segment.reverse();
        }

        self
    }

    /// Reduce the approximation to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: &CurveBoundary<Point<1>>) {
        for segment in &mut self.segments {
            segment.make_subset(boundary.normalize());
        }

        self.segments.retain(|segment| !segment.is_empty());
    }

    /// Merge the provided segment into the approximation
    pub fn merge(
        &mut self,
        new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        let mut overlapping_segments = VecDeque::new();

        let mut i = 0;
        loop {
            let Some(segment) = self.segments.get(i) else {
                break;
            };

            if segment.overlaps(&new_segment) {
                let segment = self.segments.swap_remove(i);
                overlapping_segments.push_back(segment);
                continue;
            }

            i += 1;
        }

        let mut merged_segment = new_segment;
        for segment in overlapping_segments {
            merged_segment.merge(&segment);
        }

        self.segments.push(merged_segment.clone());
        self.segments.sort();
        merged_segment
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::approx::{curve::CurveApproxSegment, ApproxPoint};

    use super::CurveApprox;

    #[test]
    fn reverse() {
        let mut approx = CurveApprox {
            segments: vec![
                CurveApproxSegment {
                    boundary: [[0.1], [0.4]].into(),
                    points: vec![
                        ApproxPoint::new([0.1], [0.1, 0.1, 0.1]),
                        ApproxPoint::new([0.4], [0.4, 0.4, 0.4]),
                    ],
                },
                CurveApproxSegment {
                    boundary: [[0.6], [0.9]].into(),
                    points: vec![
                        ApproxPoint::new([0.6], [0.6, 0.6, 0.6]),
                        ApproxPoint::new([0.9], [0.9, 0.9, 0.9]),
                    ],
                },
            ],
        };

        approx.reverse();

        assert_eq!(
            approx,
            CurveApprox {
                segments: vec![
                    CurveApproxSegment {
                        boundary: [[0.9], [0.6]].into(),
                        points: vec![
                            ApproxPoint::new([0.9], [0.9, 0.9, 0.9]),
                            ApproxPoint::new([0.6], [0.6, 0.6, 0.6]),
                        ],
                    },
                    CurveApproxSegment {
                        boundary: [[0.4], [0.1]].into(),
                        points: vec![
                            ApproxPoint::new([0.4], [0.4, 0.4, 0.4]),
                            ApproxPoint::new([0.1], [0.1, 0.1, 0.1]),
                        ],
                    },
                ],
            }
        )
    }
}
