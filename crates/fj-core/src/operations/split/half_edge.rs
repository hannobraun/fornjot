use fj_math::Point;

use crate::{
    objects::{HalfEdge, Vertex},
    operations::insert::Insert,
    services::Services,
};

/// Split a [`HalfEdge`] into two
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
        services: &mut Services,
    ) -> [HalfEdge; 2];
}

impl SplitHalfEdge for HalfEdge {
    fn split_half_edge(
        &self,
        point: impl Into<Point<1>>,
        services: &mut Services,
    ) -> [HalfEdge; 2] {
        let point = point.into();

        let [start, end] = self.boundary().inner;

        let a = HalfEdge::new(
            self.path(),
            [start, point],
            self.curve().clone(),
            self.start_vertex().clone(),
        );
        let b = HalfEdge::new(
            self.path(),
            [point, end],
            self.curve().clone(),
            Vertex::new().insert(services),
        );

        [a, b]
    }
}
