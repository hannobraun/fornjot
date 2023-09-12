use fj_math::Point;

use crate::geometry::CurveBoundary;

use super::CurveApproxSegment;

/// Partial approximation of a curve
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
        let mut existing_segment = None;
        for segment in &mut self.segments {
            if segment.overlaps(&new_segment) {
                segment.merge(&new_segment);
                existing_segment = Some(segment.clone());
            }
        }

        existing_segment.unwrap_or_else(|| {
            self.segments.push(new_segment.clone());
            new_segment
        })
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
