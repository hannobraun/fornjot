use fj_math::Point;
use itertools::Itertools;

use crate::{
    geometry::curve::Curve,
    objects::{Cycle, HalfEdge},
    operations::{BuildHalfEdge, Insert, UpdateHalfEdge},
    services::Services,
    storage::Handle,
};

/// Builder API for [`Cycle`]
#[derive(Default)]
pub struct CycleBuilder {
    half_edges: Vec<HalfEdge>,
}

impl CycleBuilder {
    /// Create a cycle whose half-edges are connected to the provided half-edges
    ///
    /// The half-edges of the new circle will be coincident with the provided
    /// half-edges, but will point in the opposite direction.
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, form a cycle themselves.
    pub fn connect_to_edges<Es>(edges: Es, services: &mut Services) -> Self
    where
        Es: IntoIterator<Item = (Handle<HalfEdge>, Curve, [Point<1>; 2])>,
        Es::IntoIter: Clone + ExactSizeIterator,
    {
        let half_edges = edges
            .into_iter()
            .circular_tuple_windows()
            .map(|((prev, _, _), (half_edge, curve, boundary))| {
                HalfEdge::unjoined(curve, boundary, services)
                    .replace_start_vertex(prev.start_vertex().clone())
                    .replace_global_form(half_edge.global_form().clone())
            })
            .collect();

        Self { half_edges }
    }

    /// Build the cycle
    pub fn build(self, services: &mut Services) -> Cycle {
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|half_edge| half_edge.insert(services));
        Cycle::new(half_edges)
    }
}
