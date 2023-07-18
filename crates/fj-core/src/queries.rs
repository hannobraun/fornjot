//! Queries about objects
//!
//! Objects have methods that provide access to anything that the object itself
//! has direct access to. However, not all potentially interesting information
//! can be accessed that way. An example are the bounding vertices of an edge:
//! `HalfEdge` only stores its starting vertex, so you need a `Cycle` to get
//! both vertices.
//!
//! This module provides traits express such non-trivial queries, and implements
//! them for various objects that have the information to answer the query.

use crate::{
    geometry::BoundingVertices,
    objects::{Cycle, Face, HalfEdge, Region, Shell},
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
        edge: &Handle<HalfEdge>,
    ) -> Option<BoundingVertices>;
}

impl BoundingVerticesOfEdge for Cycle {
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<BoundingVertices> {
        let start = edge.start_vertex().clone();
        let end = self.half_edge_after(edge)?.start_vertex().clone();

        Some(BoundingVertices::from([start, end]))
    }
}

impl BoundingVerticesOfEdge for Region {
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<BoundingVertices> {
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
        edge: &Handle<HalfEdge>,
    ) -> Option<BoundingVertices> {
        self.region().bounding_vertices_of_edge(edge)
    }
}

impl BoundingVerticesOfEdge for Shell {
    fn bounding_vertices_of_edge(
        &self,
        edge: &Handle<HalfEdge>,
    ) -> Option<BoundingVertices> {
        for face in self.faces() {
            if let Some(vertices) = face.bounding_vertices_of_edge(edge) {
                return Some(vertices);
            }
        }

        None
    }
}
