use crate::{
    kernel::geometry::{Circle, Curve, Line},
    math::{Point, Vector},
};

use super::vertices::Vertex;

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    pub edges: Vec<Edge>,
}

/// An edge of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    /// The curve that defines the edge's geometry
    ///
    /// The edge is a segment of the curve that is bounded by two vertices.
    pub curve: Curve,

    /// The vertices that bound this edge on the curve, in curve coordinates
    ///
    /// If there are no such vertices, that means the edge is connected to
    /// itself (like a full circle, for example).
    ///
    /// # Implementation note
    ///
    /// Since these vertices bound the edge, they must lie on the curve. This
    /// isn't enforced at all, however. It would make sense to store 1D vertices
    /// here, and indeed, this was the case in the past.
    ///
    /// It got in the way of some work, however, so it made sense to simplify
    /// it by storing 3D vertices. It will probably make sense to revert this
    /// and store 1D vertices again, at some point.
    pub vertices: Option<[Vertex; 2]>,
}

impl Edge {
    /// Construct an edge
    ///
    /// If vertices are provided in `vertices`, they must be on `curve`.
    ///
    /// This constructor will convert the vertices into curve coordinates. If
    /// they are not on the curve, this will result in their projection being
    /// converted into curve coordinates, which is likely not the caller's
    /// intention.
    pub fn new(curve: Curve, vertices: Option<[Vertex; 2]>) -> Self {
        Self { curve, vertices }
    }

    /// Construct an edge that is a line segment
    pub fn line_segment(vertices: [Vertex; 2]) -> Self {
        Self::new(
            Curve::Line(Line::from_points(
                vertices.clone().map(|vertex| vertex.point()),
            )),
            Some(vertices),
        )
    }

    /// Create a circle
    pub fn circle(radius: f64) -> Self {
        Self {
            curve: Curve::Circle(Circle {
                center: Point::origin(),
                radius: Vector::from([radius, 0.]),
            }),
            vertices: None,
        }
    }
}
