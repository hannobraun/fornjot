use crate::new::topology::{HalfEdge, Handle, Solid, Topology};

/// # A self-contained Fornjot model
#[derive(Debug)]
pub struct Model {
    /// # The solid that defines this model
    pub solid: Handle<Solid>,

    /// # The model's topology
    pub topology: Topology,

    /// # A list of invalid half-edges within the model
    ///
    /// ## Implementation Note
    ///
    /// This is the precursor to a more general mechanism for representing
    /// validation failures, to allow the viewer to display the details of
    /// those.
    ///
    /// For now, I decided to include just what I need, and leave that more
    /// general mechanism for another day though.
    pub invalid_half_edges: Vec<Handle<HalfEdge>>,
}
