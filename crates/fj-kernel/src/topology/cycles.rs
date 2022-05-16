use std::hash::{Hash, Hasher};

use crate::shape::{Handle, Shape};

use super::{CycleBuilder, Edge};

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
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub struct Cycle {
    /// The edges that make up the cycle
    pub edges: Vec<Handle<Edge<3>>>,
}

impl Cycle {
    /// Construct a `Cycle`
    pub fn new(edges: impl IntoIterator<Item = Handle<Edge<3>>>) -> Self {
        let edges = edges.into_iter().collect();

        Self { edges }
    }

    /// Build a cycle using the [`CycleBuilder`] API
    pub fn builder(shape: &mut Shape) -> CycleBuilder {
        CycleBuilder::new(shape)
    }

    /// Access the edges that this cycle refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn edges(&self) -> impl Iterator<Item = Edge<3>> + '_ {
        self.edges.iter().map(|handle| handle.get())
    }
}

impl PartialEq for Cycle {
    fn eq(&self, other: &Self) -> bool {
        self.edges().eq(other.edges())
    }
}

impl Hash for Cycle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for edge in self.edges() {
            edge.hash(state);
        }
    }
}
