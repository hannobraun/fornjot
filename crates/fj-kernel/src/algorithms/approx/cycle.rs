//! Cycle approximation
//!
//! See [`CycleApprox`].

use fj_math::Segment;

use crate::objects::Cycle;

use super::{
    curve::ApproxCache, edge::HalfEdgeApprox, Approx, ApproxPoint, Tolerance,
};

impl Approx for &Cycle {
    type Approximation = CycleApprox;
    type Cache = ApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        let half_edges = self
            .half_edges()
            .map(|half_edge| half_edge.approx_with_cache(tolerance, cache))
            .collect();

        CycleApprox { half_edges }
    }
}

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CycleApprox {
    /// The approximated edges that make up the approximated cycle
    pub half_edges: Vec<HalfEdgeApprox>,
}

impl CycleApprox {
    /// Compute the points that approximate the cycle
    pub fn points(&self) -> Vec<ApproxPoint<2>> {
        let mut points = Vec::new();

        for approx in &self.half_edges {
            points.extend(approx.points());
        }

        if let Some(point) = points.first() {
            points.push(point.clone());
        }

        points
    }

    /// Construct the segments that approximate the cycle
    pub fn segments(&self) -> Vec<Segment<3>> {
        let mut segments = Vec::new();

        for segment in self.points().windows(2) {
            // This can't panic, as we passed `2` to `windows`. Can be cleaned
            // up, once `array_windows` is stable.
            let segment = [&segment[0], &segment[1]];

            segments
                .push(Segment::from(segment.map(|point| point.global_form)));
        }

        segments
    }
}
