use crate::{
    objects::{handles::Handles, Face, HalfEdge},
    queries::BoundingVerticesOfHalfEdge,
    storage::Handle,
};

/// A 3-dimensional closed shell
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Shell {
    faces: Handles<Face>,
}

impl Shell {
    /// Construct an empty instance of `Shell`
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Access the faces of the shell
    pub fn faces(&self) -> &Handles<Face> {
        &self.faces
    }

    /// Indicate whether the provided half-edges are siblings
    pub fn are_siblings(
        &self,
        a: &Handle<HalfEdge>,
        b: &Handle<HalfEdge>,
    ) -> bool {
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
