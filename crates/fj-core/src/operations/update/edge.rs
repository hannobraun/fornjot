use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Curve, Edge, Vertex},
    storage::Handle,
};

/// Update a [`Edge`]
pub trait UpdateEdge {
    /// Replace the path of the edge
    #[must_use]
    fn update_path(&self, path: SurfacePath) -> Self;

    /// Replace the boundary of the edge
    #[must_use]
    fn replace_boundary(&self, boundary: CurveBoundary<Point<1>>) -> Self;

    /// Replace the curve of the edge
    #[must_use]
    fn replace_curve(&self, curve: Handle<Curve>) -> Self;

    /// Replace the start vertex of the edge
    #[must_use]
    fn replace_start_vertex(&self, start_vertex: Handle<Vertex>) -> Self;
}

impl UpdateEdge for Edge {
    fn update_path(&self, path: SurfacePath) -> Self {
        Edge::new(
            path,
            self.boundary(),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
    }

    fn replace_boundary(&self, boundary: CurveBoundary<Point<1>>) -> Self {
        Edge::new(
            self.path(),
            boundary,
            self.curve().clone(),
            self.start_vertex().clone(),
        )
    }

    fn replace_curve(&self, curve: Handle<Curve>) -> Self {
        Edge::new(
            self.path(),
            self.boundary(),
            curve,
            self.start_vertex().clone(),
        )
    }

    fn replace_start_vertex(&self, start_vertex: Handle<Vertex>) -> Self {
        Edge::new(
            self.path(),
            self.boundary(),
            self.curve().clone(),
            start_vertex,
        )
    }
}
