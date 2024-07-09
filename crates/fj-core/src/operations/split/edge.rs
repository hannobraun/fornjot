use fj_math::Point;

use crate::{
    operations::{
        geometry::UpdateHalfEdgeGeometry, insert::Insert,
        replace::ReplaceHalfEdge, split::SplitHalfEdge, update::UpdateHalfEdge,
    },
    queries::{CycleOfHalfEdge, SiblingOfHalfEdge},
    storage::Handle,
    topology::{HalfEdge, Shell},
    Core,
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
        core: &mut Core,
    ) -> (Self, [[Handle<HalfEdge>; 2]; 2]);
}

impl SplitEdge for Shell {
    fn split_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
        point: impl Into<Point<1>>,
        core: &mut Core,
    ) -> (Self, [[Handle<HalfEdge>; 2]; 2]) {
        let point = point.into();

        let sibling = self
            .get_sibling_of(half_edge)
            .expect("Expected half-edge and its sibling to be part of shell");

        let [half_edge_a, half_edge_b] = self
            .find_cycle_of_half_edge(half_edge)
            .expect("Expected half-edge to be part of shell")
            .split_half_edge(half_edge, point, core);

        let siblings = {
            let [sibling_a, sibling_b] =
                sibling.cycle.split_half_edge(&sibling.sibling, point, core);
            let sibling_b = sibling_b
                .update_start_vertex(
                    |_, _| half_edge_b.start_vertex().clone(),
                    core,
                )
                .insert(core)
                .set_geometry(
                    *core.layers.geometry.of_half_edge(&sibling_b),
                    &mut core.layers.geometry,
                );

            [sibling_a, sibling_b]
        };

        let shell = self
            .replace_half_edge(
                half_edge,
                [half_edge_a.clone(), half_edge_b.clone()],
                core,
            )
            .into_inner()
            .replace_half_edge(&sibling.sibling, siblings.clone(), core)
            .into_inner();

        (shell, [[half_edge_a, half_edge_b], siblings])
    }
}
