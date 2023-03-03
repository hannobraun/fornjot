use fj_math::Point;

use crate::{
    geometry::curve::Curve,
    objects::Vertex,
    storage::{Handle, HandleWrapper},
};

/// A directed edge, defined in a surface's 2D space
///
/// The concept of an "edge" in Fornjot is represented by two structs,
/// `HalfEdge` and [`GlobalEdge`]. `HalfEdge` has two attributes that make it
/// distinct from [`GlobalEdge`]:
///
/// - `HalfEdge` is directed, meaning it has a defined start and end vertex.
/// - `HalfEdge` is defined in the 2-dimensional space of a surface.
///
/// When multiple faces, which are bound by edges, are combined to form a solid,
/// the `HalfEdge`s that bound the face on the surface are then coincident with
/// the `HalfEdge`s of other faces, where those faces touch. Those coincident
/// `HalfEdge`s are different representations of the same edge. This edge is
/// represented by an instance of [`GlobalEdge`].
///
/// There are some requirements that a `HalfEdge` needs to uphold to be valid:
///
/// 1. Coincident `HalfEdge`s *must* refer to the same [`GlobalEdge`].
/// 2. `HalfEdge`s that are coincident, i.e. located in the same space, must
///    always be congruent. This means they must coincide *exactly*. The overlap
///    must be complete. None of the coincident `HalfEdge`s must overlap with
///    just a section of another.
///
/// That second requirement means that a `HalfEdge` might need to be split into
/// multiple smaller `HalfEdge`s that are each coincident with a `HalfEdge` in
/// another face.
///
/// # Implementation Note
///
/// There is no validation code that verifies whether coincident `HalfEdge`s
/// refer to the same [`GlobalEdge`] or not:
/// <https://github.com/hannobraun/Fornjot/issues/1594>
///
/// Conversely, there is no validation code to verify that coincident
/// `HalfEdge`s are congruent:
/// <https://github.com/hannobraun/Fornjot/issues/1608>
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    curve: Curve,
    boundary: [Point<1>; 2],
    start_vertex: Handle<Vertex>,
    global_form: HandleWrapper<GlobalEdge>,
}

impl HalfEdge {
    /// Create an instance of `HalfEdge`
    pub fn new(
        curve: Curve,
        boundary: [Point<1>; 2],
        start_vertex: Handle<Vertex>,
        global_form: Handle<GlobalEdge>,
    ) -> Self {
        Self {
            curve,
            boundary,
            start_vertex,
            global_form: global_form.into(),
        }
    }

    /// Access the curve that defines the half-edge's geometry
    pub fn curve(&self) -> Curve {
        self.curve
    }

    /// Access the boundary points of the half-edge on the curve
    pub fn boundary(&self) -> [Point<1>; 2] {
        self.boundary
    }

    /// Compute the surface position where the half-edge starts
    pub fn start_position(&self) -> Point<2> {
        // Computing the surface position from the curve position is fine.
        // `HalfEdge` "owns" its start position. There is no competing code that
        // could compute the surface position from slightly different data.

        let [start, _] = self.boundary;
        self.curve.point_from_path_coords(start)
    }

    /// Access the vertex from where this half-edge starts
    pub fn start_vertex(&self) -> &Handle<Vertex> {
        &self.start_vertex
    }

    /// Access the global form of the half-edge
    pub fn global_form(&self) -> &Handle<GlobalEdge> {
        &self.global_form
    }
}

/// An undirected edge, defined in global (3D) coordinates
///
/// In contrast to [`HalfEdge`], `GlobalEdge` is undirected, meaning it has no
/// defined direction, and its vertices have no defined order. This means it can
/// be used to determine whether two [`HalfEdge`]s map to the same `GlobalEdge`,
/// regardless of their direction.
///
/// See [`HalfEdge`]'s documentation for more information on the relationship
/// between [`HalfEdge`] and `GlobalEdge`.
#[derive(Clone, Debug, Default, Hash)]
pub struct GlobalEdge {}

impl GlobalEdge {
    /// Create a new instance
    ///
    /// The order of `vertices` is irrelevant. Two `GlobalEdge`s with the same
    /// `curve` and `vertices` will end up being equal, regardless of the order
    /// of `vertices` here.
    pub fn new() -> Self {
        Self::default()
    }
}
