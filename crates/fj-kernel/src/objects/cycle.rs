use fj_math::{Scalar, Winding};

use crate::builder::CycleBuilder;

use super::{HalfEdge, Surface};

/// A cycle of connected half-edges
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
    /// Panic, if the end of each half-edge does not connect to the beginning of
    /// the next one.
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

    /// Access the half-edges that make up the cycle
    pub fn half_edges(&self) -> impl Iterator<Item = &HalfEdge> + '_ {
        self.half_edges.iter()
    }

    /// Indicate the cycle's winding, assuming a right-handed coordinate system
    pub fn winding(&self) -> Winding {
        // The cycle could be made up of one or two circles. If that is the
        // case, the winding of the cycle is determined by the winding of the
        // first circle.
        if self.half_edges.len() < 3 {
            let first = self
                .half_edges()
                .next()
                .expect("Invalid cycle: expected at least one half-edge");

            let [a, b] = first.vertices();
            let edge_direction_positive = a.position() < b.position();

            let circle = match first.curve().kind() {
                super::CurveKind::Circle(circle) => circle,
                super::CurveKind::Line(_) => unreachable!(
                    "Invalid cycle: less than 3 edges, but not all are circles"
                ),
            };
            let cross_positive = circle.a().cross(&circle.b()) > Scalar::ZERO;

            if edge_direction_positive == cross_positive {
                return Winding::Ccw;
            } else {
                return Winding::Cw;
            }
        }

        // Now that we got the special case out of the way, we can treat the
        // cycle as a polygon:
        // https://stackoverflow.com/a/1165943

        let mut sum = Scalar::ZERO;

        for half_edge in self.half_edges.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable:
            // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
            let [a, b] = [half_edge[0], half_edge[1]];

            let [a, b] = [a, b].map(|half_edge| {
                let [vertex, _] = half_edge.vertices();
                vertex.surface_form().position()
            });

            sum += (b.u - a.u) * (b.v + a.v);
        }

        if sum > Scalar::ZERO {
            return Winding::Cw;
        }
        if sum < Scalar::ZERO {
            return Winding::Ccw;
        }

        unreachable!("Encountered invalid cycle: {self:#?}");
    }

    /// Consume the cycle and return its half-edges
    pub fn into_half_edges(self) -> impl Iterator<Item = HalfEdge> {
        self.half_edges.into_iter()
    }
}
