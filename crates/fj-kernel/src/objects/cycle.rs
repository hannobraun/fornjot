use fj_math::Point;

use super::{Edge, Surface};

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    /// The edges that make up the cycle
    pub edges: Vec<Edge>,
}

impl Cycle {
    /// Create a new cycle
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Implementation note:
        // As I'm writing this, this constructor has no arguments. I expect it
        // to take a `Surface` at some point. Remove the `#[allow(...)]`
        // attribute then.
        // - @hannobraun

        Self { edges: Vec::new() }
    }

    /// Create a polygon from a list of points
    pub fn polygon_from_points(
        surface: &Surface,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Cycle {
        let mut points: Vec<_> = points.into_iter().map(Into::into).collect();

        // A polygon is closed, so we need to add the first point at the end
        // again, for the next step.
        if let Some(point) = points.first().cloned() {
            points.push(point);
        }

        let mut edges = Vec::new();
        for points in points.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let points = [points[0], points[1]];

            edges.push(Edge::line_segment_from_points(surface, points));
        }

        Cycle { edges }
    }

    /// Access edges that make up the cycle
    pub fn edges(&self) -> impl Iterator<Item = &Edge> + '_ {
        self.edges.iter()
    }
}
