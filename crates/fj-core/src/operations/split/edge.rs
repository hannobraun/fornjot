use fj_math::Point;

use crate::{
    objects::{HalfEdge, Shell},
    operations::{
        insert::Insert, replace::ReplaceHalfEdge, split::SplitHalfEdge,
        update::UpdateHalfEdge,
    },
    queries::SiblingOfHalfEdge,
    services::Services,
    storage::Handle,
};

/// Split a pair of [`HalfEdge`]s into two
pub trait SplitEdge: Sized {
    /// Split the provided [`HalfEdge`], as well as its sibling, into two
    ///
    /// # Panics
    ///
    /// Panics, if the provided half-edge is not a part of this shell.
    #[must_use]
    fn split_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
        point: impl Into<Point<1>>,
        services: &mut Services,
    ) -> (Self, [[Handle<HalfEdge>; 2]; 2]);
}

impl SplitEdge for Shell {
    fn split_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
        point: impl Into<Point<1>>,
        services: &mut Services,
    ) -> (Self, [[Handle<HalfEdge>; 2]; 2]) {
        let point = point.into();

        let sibling = self
            .get_sibling_of(half_edge)
            .expect("Expected half-edge and its sibling to be part of shell");

        let [half_edge_a, half_edge_b] = half_edge
            .split_half_edge(point, services)
            .map(|half_edge| half_edge.insert(services));

        let siblings = {
            let [sibling_a, sibling_b] =
                sibling.split_half_edge(point, services);
            let sibling_b = sibling_b
                .update_start_vertex(|_| half_edge_b.start_vertex().clone());
            [sibling_a, sibling_b].map(|half_edge| half_edge.insert(services))
        };

        let shell = self
            .replace_half_edge(
                half_edge,
                [half_edge_a.clone(), half_edge_b.clone()],
                services,
            )
            .into_inner()
            .replace_half_edge(&sibling, siblings.clone(), services)
            .into_inner();

        (shell, [[half_edge_a, half_edge_b], siblings])
    }
}
