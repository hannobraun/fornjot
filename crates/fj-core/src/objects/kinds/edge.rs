use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Curve, Vertex},
    storage::{Handle, HandleWrapper},
};

/// A directed edge, defined in a surface's 2D space
///
/// When multiple faces, which are bound by edges, are combined to form a solid,
/// the `HalfEdge`s that bound the face on the surface are then coincident with
/// the `HalfEdge`s of other faces, where those faces touch. Those coincident
/// `HalfEdge`s are different representations of the same edge, and this fact
/// must be represented in the following way:
///
/// - The coincident `HalfEdge`s must refer to the same `Curve`.
/// - The coincident `HalfEdge`s must have the same boundary.
///
/// There is another, implicit requirement hidden here:
///
/// `HalfEdge`s that are coincident, i.e. located in the same space, must always
/// be congruent. This means they must coincide *exactly*. The overlap must be
/// complete. None of the coincident `HalfEdge`s must overlap with just a
/// section of another.
///
/// # Implementation Note
///
/// There is currently no validation code to verify that coincident `HalfEdge`s
/// are congruent:
/// <https://github.com/hannobraun/Fornjot/issues/1608>
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    path: SurfacePath,
    boundary: CurveBoundary<Point<1>>,
    curve: HandleWrapper<Curve>,
    start_vertex: HandleWrapper<Vertex>,
    global_form: HandleWrapper<GlobalEdge>,
}

impl HalfEdge {
    /// Create an instance of `HalfEdge`
    pub fn new(
        path: SurfacePath,
        boundary: impl Into<CurveBoundary<Point<1>>>,
        curve: Handle<Curve>,
        start_vertex: Handle<Vertex>,
        global_form: Handle<GlobalEdge>,
    ) -> Self {
        Self {
            path,
            boundary: boundary.into(),
            curve: curve.into(),
            start_vertex: start_vertex.into(),
            global_form: global_form.into(),
        }
    }

    /// Access the curve that defines the half-edge's geometry
    pub fn path(&self) -> SurfacePath {
        self.path
    }

    /// Access the boundary points of the half-edge on the curve
    pub fn boundary(&self) -> CurveBoundary<Point<1>> {
        self.boundary
    }

    /// Compute the surface position where the half-edge starts
    pub fn start_position(&self) -> Point<2> {
        // Computing the surface position from the curve position is fine.
        // `HalfEdge` "owns" its start position. There is no competing code that
        // could compute the surface position from slightly different data.

        let [start, _] = self.boundary.inner;
        self.path.point_from_path_coords(start)
    }

    /// Access the curve of the half-edge
    pub fn curve(&self) -> &Handle<Curve> {
        &self.curve
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
/// defined direction. This means it can be used to determine whether two
/// [`HalfEdge`]s map to the same `GlobalEdge`, regardless of their direction.
///
/// See [`HalfEdge`]'s documentation for more information on the relationship
/// between [`HalfEdge`] and `GlobalEdge`.
///
/// # Equality
///
/// `GlobalEdge` contains no data and exists purely to be used within a
/// `Handle`, where `Handle::id` can be used to compare different instances of
/// `GlobalEdge`.
///
/// If `GlobalEdge` had `Eq`/`PartialEq` implementations, it containing no data
/// would mean that all instances of `GlobalEdge` would be considered equal.
/// This would be very error-prone.
///
/// If you need to reference a `GlobalEdge` from a struct that needs to derive
/// `Eq`/`Ord`/..., you can use `HandleWrapper<GlobalEdge>` to do that. It will
/// use `Handle::id` to provide those `Eq`/`Ord`/... implementations.
#[derive(Clone, Debug, Default, Hash)]
pub struct GlobalEdge {}

impl GlobalEdge {
    /// Create a new instance
    pub fn new() -> Self {
        Self::default()
    }
}
