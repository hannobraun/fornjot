use std::slice;

use fj_interop::ext::SliceExt;
use fj_math::{Scalar, Winding};

use crate::{geometry::path::SurfacePath, storage::Handle};

use super::{HalfEdge, Surface};

/// A cycle of connected half-edges
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cycle {
    half_edges: Vec<Handle<HalfEdge>>,
}

impl Cycle {
    /// Create a new cycle
    ///
    /// # Panics
    ///
    /// Panics, if `half_edges` does not yield at least one half-edge.
    ///
    /// Panic, if the end of each half-edge does not connect to the beginning of
    /// the next one.
    pub fn new(half_edges: impl IntoIterator<Item = Handle<HalfEdge>>) -> Self {
        let half_edges = half_edges.into_iter().collect::<Vec<_>>();

        // This is not a validation check, and thus not part of the validation
        // infrastructure. The property being checked here is inherent to the
        // validity of a `Cycle`, as methods of `Cycle` might assume that there
        // is at least one edge.
        assert!(
            !half_edges.is_empty(),
            "Cycle must contain at least one half-edge"
        );

        Self { half_edges }
    }

    /// Access the surface that this cycle is in
    pub fn surface(&self) -> &Handle<Surface> {
        if let Some(half_edge) = self.half_edges.first() {
            return half_edge.surface();
        }

        unreachable!(
            "Cycle has no half-edges, which the constructor should prevent."
        )
    }

    /// Access the half-edges that make up the cycle
    pub fn half_edges(&self) -> HalfEdgesOfCycle {
        self.half_edges.iter()
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
        if self.half_edges.len() < 3 {
            let first = self
                .half_edges()
                .next()
                .expect("Invalid cycle: expected at least one half-edge");

            let [a, b] = first.vertices();
            let edge_direction_positive = a.position() < b.position();

            let circle = match first.curve().path() {
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

        for [a, b] in self.half_edges.as_slice().array_windows_ext() {
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
}

/// An iterator over the half-edges of a [`Cycle`]
///
/// Returned by [`Cycle::half_edges`].
pub type HalfEdgesOfCycle<'a> = slice::Iter<'a, Handle<HalfEdge>>;
