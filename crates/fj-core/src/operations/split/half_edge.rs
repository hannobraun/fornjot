use crate::{
    Core,
    geometry::LocalVertexGeom,
    math::Point,
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::{Cycle, HalfEdge, Vertex},
};

/// Split a [`HalfEdge`] into two
///
/// This is a low-level operation that, by itself, leaves the containing shell
/// in an invalid state. You probably want to use [`SplitEdge`] instead.
///
/// [`SplitEdge`]: super::SplitEdge
pub trait SplitHalfEdge {
    /// Split the half-edge into two
    ///
    /// # Validity
    ///
    /// Within a valid shell, a [`HalfEdge`] must have an equal but opposite
    /// sibling. This operation only splits a single half-edge, which in itself
    /// will make a valid shell invalid.
    ///
    /// The caller is responsible for also split this half-edge's sibling, if
    /// appropriate, to preserve validity.
    #[must_use]
    fn split_half_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
        point: impl Into<Point<1>>,
        core: &mut Core,
    ) -> [Handle<HalfEdge>; 2];
}

impl SplitHalfEdge for Cycle {
    fn split_half_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
        point: impl Into<Point<1>>,
        core: &mut Core,
    ) -> [Handle<HalfEdge>; 2] {
        let point = point.into();

        let a = HalfEdge::new(
            half_edge.curve().clone(),
            half_edge.start_vertex().clone(),
        )
        .insert(core)
        .derive_from(half_edge, core);
        let b = HalfEdge::new(
            half_edge.curve().clone(),
            Vertex::new().insert(core),
        )
        .insert(core)
        .derive_from(half_edge, core);

        core.layers.geometry.define_vertex(
            b.start_vertex().clone(),
            b.curve().clone(),
            LocalVertexGeom { position: point },
        );

        [a, b]
    }
}
