use crate::{
    builder::CycleBuilder,
    shape::{LocalForm, Shape},
};

use super::{Edge, Surface};

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
///
/// # Validation
///
/// A cycle that is part of a [`Shape`] must be structurally sound. That means
/// the edges it refers to, must be part of the same shape.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle<const D: usize> {
    /// The edges that make up the cycle
    pub edges: Vec<LocalForm<Edge<D>, Edge<3>>>,
}

impl Cycle<3> {
    /// Construct a `Cycle`
    pub fn new(edges: impl IntoIterator<Item = Edge<3>>) -> Self {
        let edges = edges.into_iter().map(LocalForm::canonical_only).collect();

        Self { edges }
    }

    /// Build a cycle using the [`CycleBuilder`] API
    pub fn builder(surface: Surface, shape: &mut Shape) -> CycleBuilder {
        CycleBuilder::new(surface, shape)
    }

    /// Access the edges that this cycle refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn edges(&self) -> impl Iterator<Item = Edge<3>> + '_ {
        self.edges.iter().map(|handle| handle.canonical())
    }
}
