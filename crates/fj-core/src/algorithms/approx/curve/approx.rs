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
