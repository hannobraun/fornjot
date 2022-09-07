//! Cycle approximation
//!
//! See [`CycleApprox`].

use fj_math::{Point, Segment};

use crate::objects::Cycle;

use super::{edge::EdgeApprox, Approx, Tolerance};

impl Approx for &Cycle {
    type Approximation = CycleApprox;

    fn approx(self, tolerance: Tolerance) -> Self::Approximation {
        let edges = self.edges().map(|edge| edge.approx(tolerance)).collect();
        CycleApprox { edges }
    }
}

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CycleApprox {
    /// The approximated edges that make up the approximated cycle
    pub edges: Vec<EdgeApprox>,
}

impl CycleApprox {
    /// Compute the points that approximate the cycle
    pub fn points(&self) -> Vec<(Point<2>, Point<3>)> {
        let mut points = Vec::new();

        for edge_approx in &self.edges {
            points.extend(edge_approx.points.iter().copied());
        }

        if let Some(&point) = points.first() {
            points.push(point);
        }

        points
    }

    /// Construct the segments that approximate the cycle
    pub fn segments(&self) -> Vec<Segment<3>> {
        let mut segments = Vec::new();

        for segment in self.points().windows(2) {
            // This can't panic, as we passed `2` to `windows`. Can be cleaned
            // up, once `array_windows` is stable.
            let segment = [segment[0], segment[1]];

            segments.push(Segment::from(segment.map(|point| {
                let (_, point_global) = point;
                point_global
            })));
        }

        segments
    }
}
