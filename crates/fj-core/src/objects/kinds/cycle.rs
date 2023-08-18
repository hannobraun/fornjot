use fj_math::{Scalar, Winding};
use itertools::Itertools;

use crate::{geometry::SurfacePath, objects::Edge, storage::Handle};

/// A cycle of connected half-edges
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    edges: Vec<Handle<Edge>>,
}

impl Cycle {
    /// Create an instance of `Cycle`
    pub fn new(edges: impl IntoIterator<Item = Handle<Edge>>) -> Self {
        let edges = edges.into_iter().collect::<Vec<_>>();
        Self { edges }
    }

    /// Access the half-edges that make up the cycle
    pub fn edges(&self) -> impl Iterator<Item = &Handle<Edge>> {
        self.edges.iter()
    }

    /// Access the half-edges in pairs
    pub fn edge_pairs(
        &self,
    ) -> impl Iterator<Item = (&Handle<Edge>, &Handle<Edge>)> {
        self.edges.iter().circular_tuple_windows()
    }

    /// Access the half-edge with the provided index
    pub fn nth_edge(&self, index: usize) -> Option<&Handle<Edge>> {
        self.edges.get(index)
    }

    /// Access the half-edge after the provided one
    ///
    /// Returns `None`, if the provided [`Edge`] is not part of the cycle.
    pub fn edge_after(&self, edge: &Handle<Edge>) -> Option<&Handle<Edge>> {
        self.index_of(edge).map(|index| {
            let next_index = (index + 1) % self.edges.len();
            &self.edges[next_index]
        })
    }

    /// Return the index of the provided half-edge, if it is in this cycle
    pub fn index_of(&self, half_edge: &Handle<Edge>) -> Option<usize> {
        self.edges.iter().position(|e| e.id() == half_edge.id())
    }

    /// Return the number of half-edges in the cycle
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Indicate whether the cycle is empty
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Indicate the cycle's winding, assuming a right-handed coordinate system
    ///
    /// Please note that this is not *the* winding of the cycle, only one of the
    /// two possible windings, depending on the direction you look at the
    /// surface that the cycle is defined on from.
    pub fn winding(&self) -> Winding {
        // The cycle could be made up of one or two circles. If that is the
        // case, the winding of the cycle is determined by the winding of the
        // first circle.
        if self.edges.len() < 3 {
            let first = self
                .edges()
                .next()
                .expect("Invalid cycle: expected at least one half-edge");

            let [a, b] = first.boundary().inner;
            let edge_direction_positive = a < b;

            let circle = match first.path() {
                SurfacePath::Circle(circle) => circle,
                SurfacePath::Line(_) => unreachable!(
                    "Invalid cycle: less than 3 edges, but not all are circles"
                ),
            };
            let cross_positive = circle.a().cross2d(&circle.b()) > Scalar::ZERO;

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

        for (a, b) in self.edge_pairs() {
            let [a, b] = [a, b].map(|half_edge| half_edge.start_position());

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
}
