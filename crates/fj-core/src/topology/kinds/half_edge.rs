use crate::{
    storage::Handle,
    topology::{Curve, Vertex},
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
/// [`Cycle`]: crate::topology::Cycle
/// [`Shell`]: crate::topology::Shell
#[derive(Clone, Debug)]
pub struct HalfEdge {
    curve: Handle<Curve>,
    start_vertex: Handle<Vertex>,
}

impl HalfEdge {
    /// Create an instance of `Edge`
    pub fn new(curve: Handle<Curve>, start_vertex: Handle<Vertex>) -> Self {
        Self {
            curve,
            start_vertex,
        }
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
