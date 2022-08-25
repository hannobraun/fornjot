use crate::builder::CycleBuilder;

use super::{Edge, Surface};

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    surface: Surface,
    edges: Vec<Edge>,
}

impl Cycle {
    /// Build a cycle using [`CycleBuilder`]
    pub fn build(surface: Surface) -> CycleBuilder {
        CycleBuilder::new(surface)
    }

    /// Create a new cycle
    pub fn new(surface: Surface) -> Self {
        Self {
            surface,
            edges: Vec::new(),
        }
    }

    /// Add edges to the cycle
    ///
    /// Consumes the cycle and returns the updated instance.
    pub fn with_edges(mut self, edges: impl IntoIterator<Item = Edge>) -> Self {
        self.edges.extend(edges);
        self
    }

    /// Access the surface that this cycle is in
    pub fn surface(&self) -> &Surface {
        &self.surface
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
