use crate::new::topology::{Handle, Solid, Topology};

/// # A self-contained Fornjot model
pub struct Model {
    /// # The solid that defines this model
    pub solid: Handle<Solid>,

    /// # The model's topology
    pub topology: Topology,
}
