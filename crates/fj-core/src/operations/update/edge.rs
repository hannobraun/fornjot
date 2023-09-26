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
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
    ) -> Self;

    /// Replace the boundary of the edge
    #[must_use]
    fn update_boundary(
        &self,
        update: impl FnOnce(CurveBoundary<Point<1>>) -> CurveBoundary<Point<1>>,
    ) -> Self;

    /// Replace the curve of the edge
    #[must_use]
    fn update_curve(
        &self,
        update: impl FnOnce(&Handle<Curve>) -> Handle<Curve>,
    ) -> Self;

    /// Replace the start vertex of the edge
    #[must_use]
    fn update_start_vertex(
        &self,
        update: impl FnOnce(&Handle<Vertex>) -> Handle<Vertex>,
    ) -> Self;
}

impl UpdateEdge for Edge {
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
    ) -> Self {
        Edge::new(
            update(self.path()),
            self.boundary(),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
    }

    fn update_boundary(
        &self,
        update: impl FnOnce(CurveBoundary<Point<1>>) -> CurveBoundary<Point<1>>,
    ) -> Self {
        Edge::new(
            self.path(),
            update(self.boundary()),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
    }

    fn update_curve(
        &self,
        update: impl FnOnce(&Handle<Curve>) -> Handle<Curve>,
    ) -> Self {
        Edge::new(
            self.path(),
            self.boundary(),
            update(self.curve()),
            self.start_vertex().clone(),
        )
    }

    fn update_start_vertex(
        &self,
        update: impl FnOnce(&Handle<Vertex>) -> Handle<Vertex>,
    ) -> Self {
        Edge::new(
            self.path(),
            self.boundary(),
            self.curve().clone(),
            update(self.start_vertex()),
        )
    }
}
