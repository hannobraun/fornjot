use fj_math::{Point, Segment};

use crate::{local::Local, objects::Cycle};

use super::{curves::approx_curve, edges::approx_edge, Tolerance};

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CycleApprox {
    /// The points that approximate the cycle
    pub points: Vec<Local<Point<2>>>,
}

impl CycleApprox {
    /// Compute the approximation of a cycle
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual face.
    pub fn new(cycle: &Cycle, tolerance: Tolerance) -> Self {
        let mut points = Vec::new();

        for edge in &cycle.edges {
            let mut edge_points = Vec::new();
            approx_curve(edge.curve().global(), tolerance, &mut edge_points);
            approx_edge(*edge.vertices(), &mut edge_points);

            points.extend(edge_points.into_iter().map(|point| {
                let local = edge
                    .curve()
                    .local()
                    .point_from_curve_coords(*point.local());
                Local::new(local, *point.global())
            }));
        }

        // Can't just rely on `dedup`, as the conversion from curve coordinates
        // could lead to subtly different surface coordinates.
        points.dedup_by(|a, b| a.global() == b.global());

        Self { points }
    }

    /// Construct the segments that approximate the cycle
    pub fn segments(&self) -> Vec<Segment<3>> {
        let mut segments = Vec::new();

        for segment in self.points.windows(2) {
            // This can't panic, as we passed `2` to `windows`. Can be cleaned
            // up, once `array_windows` is stable.
            let segment = [segment[0], segment[1]];

            segments.push(Segment::from(segment.map(|point| *point.global())));
        }

        segments
    }
}
