use fj_math::Segment;

use crate::{geometry::LocalPoint, objects::Cycle};

use super::{curves::approx_curve, edges::approx_edge, Tolerance};

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CycleApprox {
    /// The points that approximate the cycle
    pub points: Vec<LocalPoint<3>>,
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
            approx_curve(&edge.curve(), tolerance, &mut edge_points);
            approx_edge(edge.vertices.clone(), &mut edge_points);

            points.extend(edge_points);
        }

        let mut points: Vec<_> = points
            .into_iter()
            .map(|point| LocalPoint::new(point.global(), point.global()))
            .collect();

        points.dedup();

        Self { points }
    }

    /// Construct the segments that approximate the cycle
    pub fn segments(&self) -> Vec<Segment<3>> {
        let mut segments = Vec::new();

        for segment in self.points.windows(2) {
            // This can't panic, as we passed `2` to `windows`. Can be cleaned
            // up, once `array_windows` is stable.
            let segment = [segment[0], segment[1]];

            let segment = segment.map(|point| point.global());
            segments.push(Segment::from(segment));
        }

        segments
    }
}
