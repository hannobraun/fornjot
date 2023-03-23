use crate::{
    objects::{GlobalEdge, HalfEdge},
    storage::Handle,
};

/// Update a [`HalfEdge`]
pub trait UpdateHalfEdge {
    /// Update the global form of the half-edge
    fn update_global_form(&self, global_form: Handle<GlobalEdge>) -> HalfEdge;
}

impl UpdateHalfEdge for HalfEdge {
    fn update_global_form(&self, global_form: Handle<GlobalEdge>) -> HalfEdge {
        HalfEdge::new(
            self.curve(),
            self.boundary(),
            self.start_vertex().clone(),
            global_form,
        )
    }
}
