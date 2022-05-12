use fj_math::{Point, Segment};

use crate::topology::Cycle;

use super::{curves::approx_curve, edges::approximate_edge, Tolerance};

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CycleApprox {
    /// The points that approximate the cycle
    pub points: Vec<Point<3>>,
}

impl CycleApprox {
    /// Compute the approximation of a cycle
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual face.
    pub fn new(cycle: &Cycle, tolerance: Tolerance) -> Self {
        let mut points = Vec::new();

        for edge in cycle.edges() {
            let mut edge_points = Vec::new();
            approx_curve(&edge.curve(), tolerance, &mut edge_points);

            points.extend(approximate_edge(edge_points, edge.vertices()));
        }

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

            segments.push(Segment::from(segment));
        }

        segments
    }
}

impl<T, P> From<T> for CycleApprox
where
    T: IntoIterator<Item = P>,
    P: Into<Point<3>>,
{
    fn from(points: T) -> Self {
        let points = points.into_iter().map(Into::into).collect();
        Self { points }
    }
}
