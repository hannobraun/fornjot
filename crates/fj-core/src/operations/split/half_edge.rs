use fj_math::Point;

use crate::{
    geometry::LocalVertexGeom,
    operations::{
        derive::DeriveFrom, geometry::UpdateHalfEdgeGeometry, insert::Insert,
    },
    storage::Handle,
    topology::{HalfEdge, Vertex},
    Core,
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
        point: impl Into<Point<1>>,
        core: &mut Core,
    ) -> [Handle<HalfEdge>; 2];
}

impl SplitHalfEdge for Handle<HalfEdge> {
    fn split_half_edge(
        &self,
        point: impl Into<Point<1>>,
        core: &mut Core,
    ) -> [Handle<HalfEdge>; 2] {
        let point = point.into();

        let geometry = *core.layers.geometry.of_half_edge(self);
        let [start, end] = geometry.boundary.inner;

        let a =
            HalfEdge::new(self.curve().clone(), self.start_vertex().clone())
                .insert(core)
                .derive_from(self, core)
                .set_geometry(
                    geometry.with_boundary([start, point]),
                    &mut core.layers.geometry,
                );
        let b = HalfEdge::new(self.curve().clone(), Vertex::new().insert(core))
            .insert(core)
            .derive_from(self, core)
            .set_geometry(
                geometry.with_boundary([point, end]),
                &mut core.layers.geometry,
            );

        core.layers.geometry.define_vertex(
            b.start_vertex().clone(),
            b.curve().clone(),
            LocalVertexGeom { position: point },
        );

        [a, b]
    }
}
