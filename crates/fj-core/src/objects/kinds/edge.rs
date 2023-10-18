use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Curve, Vertex},
    storage::{Handle, HandleWrapper},
};

/// A directed edge, defined in a surface's 2D space
///
/// When multiple faces, which are bound by edges, are combined to form a solid,
/// the `Edge`s that bound the face on the surface are then coincident with the
/// `Edge`s of other faces, where those faces touch. Such coincident `Edge`s
///  must always refer to the same `Curve`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    path: SurfacePath,
    boundary: CurveBoundary<Point<1>>,
    curve: HandleWrapper<Curve>,
    start_vertex: HandleWrapper<Vertex>,
}

impl HalfEdge {
    /// Create an instance of `Edge`
    pub fn new(
        path: SurfacePath,
        boundary: impl Into<CurveBoundary<Point<1>>>,
        curve: Handle<Curve>,
        start_vertex: Handle<Vertex>,
    ) -> Self {
        Self {
            path,
            boundary: boundary.into(),
            curve: curve.into(),
            start_vertex: start_vertex.into(),
        }
    }

    /// Access the curve that defines the edge's geometry
    pub fn path(&self) -> SurfacePath {
        self.path
    }

    /// Access the boundary points of the edge on the curve
    pub fn boundary(&self) -> CurveBoundary<Point<1>> {
        self.boundary
    }

    /// Compute the surface position where the edge starts
    pub fn start_position(&self) -> Point<2> {
        // Computing the surface position from the curve position is fine.
        // `Edge` "owns" its start position. There is no competing code that
        // could compute the surface position from slightly different data.

        let [start, _] = self.boundary.inner;
        self.path.point_from_path_coords(start)
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
