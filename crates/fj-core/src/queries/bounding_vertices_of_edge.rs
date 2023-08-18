use crate::{
    geometry::CurveBoundary,
    objects::{Cycle, Edge, Face, Region, Shell, Vertex},
    storage::Handle,
};

/// Determine the bounding vertices of an edge
pub trait BoundingVerticesOfEdge {
    /// Determine the bounding vertices of an edge
    ///
    /// Returns `None`, if the provided edge is not part of the object this
    /// method is called on.
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<Edge>,
    ) -> Option<CurveBoundary<Vertex>>;
}

impl BoundingVerticesOfEdge for Cycle {
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<Edge>,
    ) -> Option<CurveBoundary<Vertex>> {
        let start = edge.start_vertex().clone();
        let end = self.half_edge_after(edge)?.start_vertex().clone();

        Some(CurveBoundary::from([start, end]))
    }
}

impl BoundingVerticesOfEdge for Region {
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<Edge>,
    ) -> Option<CurveBoundary<Vertex>> {
        for cycle in self.all_cycles() {
            if let Some(vertices) = cycle.bounding_vertices_of_edge(edge) {
                return Some(vertices);
            }
        }

        None
    }
}

impl BoundingVerticesOfEdge for Face {
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<Edge>,
    ) -> Option<CurveBoundary<Vertex>> {
        self.region().bounding_vertices_of_edge(edge)
    }
}

impl BoundingVerticesOfEdge for Shell {
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<Edge>,
    ) -> Option<CurveBoundary<Vertex>> {
        for face in self.faces() {
            if let Some(vertices) = face.bounding_vertices_of_edge(edge) {
                return Some(vertices);
            }
        }

        None
    }
}
