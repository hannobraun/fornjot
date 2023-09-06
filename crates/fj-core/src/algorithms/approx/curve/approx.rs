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
        for segment in &mut self.segments {
            segment.reverse();
        }

        self
    }
}
