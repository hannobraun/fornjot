use crate::builder::CycleBuilder;

use super::{Edge, Surface};

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    edges: Vec<Edge>,
}

impl Cycle {
    /// Create a new cycle
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Implementation note:
        // As I'm writing this, this constructor has no arguments. I expect it
        // to take a `Surface` at some point. Remove the `#[allow(...)]`
        // attribute then.
        // - @hannobraun

        Self { edges: Vec::new() }
    }

    /// Add edges to the cycle
    ///
    /// Consumes the cycle and returns the updated instance.
    pub fn with_edges(mut self, edges: impl IntoIterator<Item = Edge>) -> Self {
        self.edges.extend(edges);
        self
    }

    /// Build a cycle using [`CycleBuilder`]
    pub fn build(surface: Surface) -> CycleBuilder {
        CycleBuilder::new(surface)
    }

    /// Access edges that make up the cycle
    pub fn edges(&self) -> impl Iterator<Item = &Edge> + '_ {
        self.edges.iter()
    }

    /// Consume the cycle and return its edges
    pub fn into_edges(self) -> impl Iterator<Item = Edge> {
        self.edges.into_iter()
    }
}
