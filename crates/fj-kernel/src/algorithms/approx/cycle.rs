use fj_math::{Point, Segment};

use crate::objects::Cycle;

use super::{Approx, Tolerance};

impl Approx for Cycle {
    type Approximation = CycleApprox;
    type Params = ();

    fn approx(
        &self,
        tolerance: Tolerance,
        (): Self::Params,
    ) -> Self::Approximation {
        let mut points = Vec::new();

        for edge in self.edges() {
            let edge_points = edge.approx(tolerance, ());
            points.extend(edge_points);
        }

        if let Some(&point) = points.first() {
            points.push(point);
        }

        CycleApprox { points }
    }
}

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CycleApprox {
    /// The points that approximate the cycle
    pub points: Vec<(Point<2>, Point<3>)>,
}

impl CycleApprox {
    /// Construct the segments that approximate the cycle
    pub fn segments(&self) -> Vec<Segment<3>> {
        let mut segments = Vec::new();

        for segment in self.points.windows(2) {
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
