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

    /// Merge the provided segment into the approximation
    pub fn merge(
        &mut self,
        new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        // We assume that approximated curve segments never overlap, unless they
        // are completely congruent. As a consequence of this, we don't have to
        // do any merging with existing segments here.
        //
        // For now, this is a valid assumption, as it matches the uses of this
        // method, due to documented limitations elsewhere in the system.

        let mut existing_segment = None;
        for segment in self.segments.iter().cloned() {
            if segment.boundary == new_segment.boundary {
                existing_segment = Some(segment);
            }
        }

        match existing_segment {
            Some(segment) => segment,
            None => {
                self.segments.push(new_segment.clone());
                new_segment
            }
        }
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
