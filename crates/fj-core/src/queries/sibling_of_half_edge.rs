use crate::{
    storage::Handle,
    topology::{HalfEdge, Shell},
};

use super::BoundingVerticesOfHalfEdge;

/// Queries related to the sibling of a [`HalfEdge`]
pub trait SiblingOfHalfEdge {
    /// Indicate whether the provided half-edges are siblings
    fn are_siblings(&self, a: &Handle<HalfEdge>, b: &Handle<HalfEdge>) -> bool;

    /// Retrieve the sibling of this half-edge
    ///
    /// Returns `None`, if the provided half-edge is not part of the object this
    /// method is called on, or if the provided half-edge has no sibling within
    /// the object.
    fn get_sibling_of(&self, half_edge: &Handle<HalfEdge>) -> Option<Sibling>;
}

impl SiblingOfHalfEdge for Shell {
    fn are_siblings(&self, a: &Handle<HalfEdge>, b: &Handle<HalfEdge>) -> bool {
        let same_curve = a.curve().id() == b.curve().id();
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

        same_curve && same_vertices
    }

    fn get_sibling_of(&self, half_edge: &Handle<HalfEdge>) -> Option<Sibling> {
        for face in self.faces() {
            for cycle in face.region().all_cycles() {
                for h in cycle.half_edges() {
                    if self.are_siblings(half_edge, h) {
                        return Some(Sibling { sibling: h.clone() });
                    }
                }
            }
        }

        None
    }
}

/// The sibling of a half-edge, plus some extra information
pub struct Sibling {
    /// The sibling
    pub sibling: Handle<HalfEdge>,
}
