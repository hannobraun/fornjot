use fj_math::{Point, Segment};

use crate::objects::Cycle;

use super::{Approx, Tolerance};

impl Approx for Cycle {
    type Approximation = CycleApprox;

    fn approx(&self, tolerance: Tolerance) -> Self::Approximation {
        let mut points = Vec::new();

        for edge in self.edges() {
            let edge_points = edge.approx(tolerance);

            points.extend(edge_points.into_iter().map(|point| {
                let (point_curve, point_global) = point;

                let point_surface =
                    edge.curve().kind().point_from_curve_coords(point_curve);
                (point_surface, point_global)
            }));
        }

        // Can't just rely on `dedup`, as the conversion from curve coordinates
        // could lead to subtly different surface coordinates.
        points.dedup_by(|(_, a), (_, b)| a == b);

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
