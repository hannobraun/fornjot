use crate::builder::CycleBuilder;

use super::{HalfEdge, Surface};

/// A cycle of connected edges
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    surface: Surface,
    half_edges: Vec<HalfEdge>,
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
        half_edges: impl IntoIterator<Item = HalfEdge>,
    ) -> Self {
        let half_edges = half_edges.into_iter().collect::<Vec<_>>();

        // Verify, that the curves of all edges are defined in the correct
        // surface.
        for edge in &half_edges {
            assert_eq!(
                &surface,
                edge.curve().surface(),
                "Edges in cycle not defined in same surface"
            );
        }

        if half_edges.len() != 1 {
            // If the length is one, we might have a cycle made up of just one
            // circle. If that isn't the case, we are dealing with line segments
            // and can be sure that the following `get_or_panic` calls won't
            // panic.

            // Verify that all edges connect.
            for half_edges in half_edges.windows(2) {
                // Can't panic, as we passed `2` to `windows`.
                //
                // Can be cleaned up, once `array_windows` is stable"
                // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
                let [a, b] = [&half_edges[0], &half_edges[1]];

                let [_, prev] = a.vertices();
                let [next, _] = b.vertices();

                assert_eq!(
                    prev.surface_form(),
                    next.surface_form(),
                    "Edges in cycle do not connect"
                );
            }

            // Verify that the edges form a cycle
            if let Some(first) = half_edges.first() {
                if let Some(last) = half_edges.last() {
                    let [first, _] = first.vertices();
                    let [_, last] = last.vertices();

                    assert_eq!(
                        first.surface_form(),
                        last.surface_form(),
                        "Edges do not form a cycle"
                    );
                }
            }
        }

        Self {
            surface,
            half_edges,
        }
    }

    /// Access the surface that this cycle is in
    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    /// Access edges that make up the cycle
    pub fn edges(&self) -> impl Iterator<Item = &HalfEdge> + '_ {
        self.half_edges.iter()
    }

    /// Consume the cycle and return its edges
    pub fn into_edges(self) -> impl Iterator<Item = HalfEdge> {
        self.half_edges.into_iter()
    }
}
