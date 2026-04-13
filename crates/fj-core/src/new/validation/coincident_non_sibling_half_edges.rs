use crate::new::topology::{HalfEdge, Handle};

/// # A shell contains coincident half-edges that are no siblings
///
/// [`HalfEdge`]s are considered siblings, if they reference the same [`Edge`].
/// Coincident half-edges within the same shell must be siblings, for the shell
/// to be valid.
///
/// [`Edge`]: crate::new::topology::Edge
pub struct CoincidentNonSiblingHalfEdges {
    /// # The coincident, non-sibling half-edges
    pub half_edges: [Handle<HalfEdge>; 2],
}
