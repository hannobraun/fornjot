use fj_math::{Line, Point};

use crate::shape::LocalForm;

use super::{Curve, Edge, Surface};

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    /// The edges that make up the cycle
    pub edges: Vec<LocalForm<Edge<2>, Edge<3>>>,
}

impl Cycle {
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

            let points_canonical =
                points.map(|point| surface.point_from_surface_coords(point));
            let edge_canonical =
                Edge::line_segment_from_points(points_canonical);

            let edge_local = Edge {
                curve: LocalForm::new(
                    Curve::Line(Line::from_points(points)),
                    *edge_canonical.curve.canonical(),
                ),
                vertices: edge_canonical.vertices.clone(),
            };

            edges.push(LocalForm::new(edge_local, edge_canonical));
        }

        Cycle { edges }
    }

    /// Access the edges that this cycle refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn edges(&self) -> impl Iterator<Item = Edge<3>> + '_ {
        self.edges.iter().map(|handle| handle.canonical().clone())
    }
}
