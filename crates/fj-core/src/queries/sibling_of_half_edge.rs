use crate::{
    objects::{HalfEdge, Shell},
    storage::Handle,
};

use super::BoundingVerticesOfHalfEdge;

/// Queries related to the sibling of a [`HalfEdge`]
pub trait SiblingOfHalfEdge {
    /// Indicate whether the provided half-edges are siblings
    fn are_siblings(&self, a: &Handle<HalfEdge>, b: &Handle<HalfEdge>) -> bool;
}

impl SiblingOfHalfEdge for Shell {
    fn are_siblings(&self, a: &Handle<HalfEdge>, b: &Handle<HalfEdge>) -> bool {
        let same_curve = a.curve().id() == b.curve().id();
        let same_boundary = a.boundary() == b.boundary().reverse();
        let same_vertices = {
            let Some(a_vertices) = self.bounding_vertices_of_half_edge(a)
            else {
                return false;
            };
            let Some(b_vertices) = self.bounding_vertices_of_half_edge(b)
            else {
                return false;
            };

            a_vertices == b_vertices.reverse()
        };

        same_curve && same_boundary && same_vertices
    }
}
