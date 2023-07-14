use crate::{
    objects::{GlobalEdge, HalfEdge, Vertex},
    storage::Handle,
};

/// Update a [`HalfEdge`]
pub trait UpdateHalfEdge {
    /// Update the start vertex of the half-edge
    #[must_use]
    fn replace_start_vertex(&self, start_vertex: Handle<Vertex>) -> Self;

    /// Update the global form of the half-edge
    #[must_use]
    fn replace_global_form(&self, global_form: Handle<GlobalEdge>) -> Self;
}

impl UpdateHalfEdge for HalfEdge {
    fn replace_start_vertex(&self, start_vertex: Handle<Vertex>) -> Self {
        HalfEdge::new(
            self.path(),
            self.boundary(),
            self.curve().clone(),
            start_vertex,
            self.global_form().clone(),
        )
    }

    fn replace_global_form(&self, global_form: Handle<GlobalEdge>) -> Self {
        HalfEdge::new(
            self.path(),
            self.boundary(),
            self.curve().clone(),
            self.start_vertex().clone(),
            global_form,
        )
    }
}
