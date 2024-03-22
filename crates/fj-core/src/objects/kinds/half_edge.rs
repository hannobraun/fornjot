use fj_math::Point;

use crate::{
    geometry::CurveBoundary,
    objects::{Curve, Vertex},
    storage::{Handle, HandleWrapper},
};

/// # A directed half-edge, defined in a surface's 2D space
///
/// ## Structure
///
/// A `HalfEdge` is defined by the [`Curve`] it is on, its boundary on the
/// curve, and the [`Vertex`] instances that bound it on the curve. To keep the
/// data structures simple (by avoiding redundancy), each `HalfEdge` only refers
/// to its start vertex. The vertex where it ends is referred to by the next
/// `HalfEdge` in the [`Cycle`] that the `HalfEdge` is a part of.
///
///
/// ## Validity
///
/// A valid `HalfEdge` must have a non-zero length, meaning its bounding
/// vertices must not be coincident.
///
/// In a valid [`Shell`], `HalfEdge`s form coincident pairs, where the faces of
/// the shell touch. The other `HalfEdge` in such a pair is called the sibling.
///
/// A `HalfEdge` and its sibling are equal but opposite. Specifically this means
/// that both refer to the same curve; that the sibling has the same, but
/// inverted, boundary; and that both are bound by the same vertices, though
/// their start vertices are different.
///
///
/// [`Cycle`]: crate::objects::Cycle
/// [`Shell`]: crate::objects::Shell
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    boundary: CurveBoundary<Point<1>>,
    curve: HandleWrapper<Curve>,
    start_vertex: HandleWrapper<Vertex>,
}

impl HalfEdge {
    /// Create an instance of `Edge`
    pub fn new(
        boundary: impl Into<CurveBoundary<Point<1>>>,
        curve: Handle<Curve>,
        start_vertex: Handle<Vertex>,
    ) -> Self {
        Self {
            boundary: boundary.into(),
            curve: curve.into(),
            start_vertex: start_vertex.into(),
        }
    }

    /// Access the boundary points of the edge on the curve
    pub fn boundary(&self) -> CurveBoundary<Point<1>> {
        self.boundary
    }

    /// Access the curve of the edge
    pub fn curve(&self) -> &Handle<Curve> {
        &self.curve
    }

    /// Access the vertex from where this edge starts
    pub fn start_vertex(&self) -> &Handle<Vertex> {
        &self.start_vertex
    }
}
