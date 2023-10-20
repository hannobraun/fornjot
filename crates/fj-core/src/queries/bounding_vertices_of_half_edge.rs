use crate::{
    geometry::CurveBoundary,
    objects::{Cycle, Face, HalfEdge, Region, Shell, Vertex},
    storage::Handle,
};

/// Determine the bounding vertices of a half-edge
pub trait BoundingVerticesOfHalfEdge {
    /// Determine the bounding vertices of a half-edge
    ///
    /// Returns `None`, if the provided half-edge is not part of the object this
    /// method is called on.
    fn bounding_vertices_of_half_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<CurveBoundary<Vertex>>;
}

impl BoundingVerticesOfHalfEdge for Cycle {
    fn bounding_vertices_of_half_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<CurveBoundary<Vertex>> {
        let start = edge.start_vertex().clone();
        let end = self.half_edges().after(edge)?.start_vertex().clone();

        Some(CurveBoundary::from([start, end]))
    }
}

impl BoundingVerticesOfHalfEdge for Region {
    fn bounding_vertices_of_half_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<CurveBoundary<Vertex>> {
        for cycle in self.all_cycles() {
            if let Some(vertices) = cycle.bounding_vertices_of_half_edge(edge) {
                return Some(vertices);
            }
        }

        None
    }
}

impl BoundingVerticesOfHalfEdge for Face {
    fn bounding_vertices_of_half_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<CurveBoundary<Vertex>> {
        self.region().bounding_vertices_of_half_edge(edge)
    }
}

impl BoundingVerticesOfHalfEdge for Shell {
    fn bounding_vertices_of_half_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<CurveBoundary<Vertex>> {
        for face in self.faces() {
            if let Some(vertices) = face.bounding_vertices_of_half_edge(edge) {
                return Some(vertices);
            }
        }

        None
    }
}
