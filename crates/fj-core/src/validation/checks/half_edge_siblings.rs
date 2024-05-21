use crate::{storage::Handle, topology::HalfEdge};

/// A [`Shell`] contains a [`HalfEdge`] without a sibling
///
/// Half-edges that are coincident must reference the same curve. This makes
/// those half-edges siblings.
///
/// In a shell, every half-edge must have a sibling. If that is not the case,
/// this is a sign of either of the following:
/// - That the shell is not closed, meaning it has some kind of hole.
/// - If the shell is closed, that its topological object graph is not valid.
///
/// [`Shell`]: crate::topology::Shell
#[derive(Clone, Debug, thiserror::Error)]
#[error("Half-edge has no sibling: {half_edge:#?}")]
pub struct HalfEdgeHasNoSibling {
    /// The half-edge that does not have a sibling
    pub half_edge: Handle<HalfEdge>,
}
