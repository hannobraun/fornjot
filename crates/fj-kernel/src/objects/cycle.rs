use fj_math::{Line, Point};

use crate::shape::LocalForm;

use super::{Curve, Edge, Surface};

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle<const D: usize> {
    /// The edges that make up the cycle
    pub edges: Vec<LocalForm<Edge<D>, Edge<3>>>,
}

impl Cycle<2> {
    /// Temporary utility method to aid refactoring
    pub fn to_canonical(&self) -> Cycle<3> {
        let mut edges = Vec::new();

        for edge in &self.edges {
            let edge = edge.local().to_canonical();
            let edge = LocalForm::canonical_only(edge);
            edges.push(edge);
        }

        Cycle { edges }
    }
}

impl Cycle<3> {
    /// Construct a `Cycle`
    pub fn new(edges: impl IntoIterator<Item = Edge<3>>) -> Self {
        let edges = edges.into_iter().map(LocalForm::canonical_only).collect();

        Self { edges }
    }

    /// Create a polygon from a list of points
    pub fn polygon_from_points(
        surface: &Surface,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Cycle<2> {
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
