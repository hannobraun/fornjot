use crate::builder::CycleBuilder;

use super::{Edge, Surface};

/// A cycle of connected edges
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
    ///
    /// # Panics
    ///
    /// Panic, if the end of each edge does not connect to the beginning of the
    /// next edge.
    pub fn new(
        surface: Surface,
        edges: impl IntoIterator<Item = Edge>,
    ) -> Self {
        let edges = edges.into_iter().collect::<Vec<_>>();

        // Verify, that the curves of all edges are defined in the correct
        // surface.
        for edge in &edges {
            assert_eq!(
                &surface,
                edge.curve().surface(),
                "Edges in cycle not defined in same surface"
            );
        }

        if edges.len() != 1 {
            // If the length is one, we might have a cycle made up of just one
            // circle. If that isn't the case, we are dealing with line segments
            // and can be sure that the following `get_or_panic` calls won't
            // panic.

            // Verify that all edges connect.
            for edges in edges.windows(2) {
                // Can't panic, as we passed `2` to `windows`.
                //
                // Can be cleaned up, once `array_windows` is stable"
                // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
                let [a, b] = [&edges[0], &edges[1]];

                let [_, prev] = a.vertices().get_or_panic();
                let [next, _] = b.vertices().get_or_panic();

                assert_eq!(
                    prev.surface_form(),
                    next.surface_form(),
                    "Edges in cycle do not connect"
                );
            }

            // Verify that the edges form a cycle
            if let Some(first) = edges.first() {
                if let Some(last) = edges.last() {
                    let [first, _] = first.vertices().get_or_panic();
                    let [_, last] = last.vertices().get_or_panic();

                    assert_eq!(
                        first.surface_form(),
                        last.surface_form(),
                        "Edges do not form a cycle"
                    );
                }
            }
        }

        Self { surface, edges }
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
