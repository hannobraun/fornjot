use fj_math::Point;

use crate::{
    geometry::HalfEdgeGeometry,
    objects::{HalfEdge, Vertex},
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
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

        let [start, end] = self.boundary().inner;

        let a = HalfEdge::new(
            [start, point],
            self.curve().clone(),
            self.start_vertex().clone(),
        )
        .insert(core)
        .derive_from(self, core);
        let b = HalfEdge::new(
            [point, end],
            self.curve().clone(),
            Vertex::new().insert(core),
        )
        .insert(core)
        .derive_from(self, core);

        core.layers.geometry.define_half_edge(
            a.clone(),
            HalfEdgeGeometry {
                path: core.layers.geometry.of_half_edge(self).path,
            },
        );
        core.layers.geometry.define_half_edge(
            b.clone(),
            HalfEdgeGeometry {
                path: core.layers.geometry.of_half_edge(self).path,
            },
        );

        [a, b]
    }
}
