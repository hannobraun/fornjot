use fj_math::Point;

use crate::{
    objects::{Cycle, HalfEdge, Surface},
    stores::Stores,
};

/// API for building a [`Cycle`]
///
/// Also see [`Cycle::builder`].
pub struct CycleBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Cycle`] is defined in
    pub surface: Surface,

    /// The half-edges that make up the [`Cycle`]
    pub half_edges: Vec<HalfEdge>,
}

impl<'a> CycleBuilder<'a> {
    /// Build the [`Cycle`] with the given half-edge
    pub fn with_half_edges(
        mut self,
        half_edge: impl IntoIterator<Item = HalfEdge>,
    ) -> Self {
        self.half_edges.extend(half_edge);
        self
    }

    /// Build the [`Cycle`] with a polygonal chain from the provided points
    pub fn with_poly_chain_from_points(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let points = self
            .half_edges
            .last()
            .map(|half_edge| {
                let [_, last] = half_edge.vertices();
                last.surface_form().position()
            })
            .into_iter()
            .chain(points.into_iter().map(Into::into))
            .collect::<Vec<_>>();

        for points in points.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let points = [points[0], points[1]];

            self.half_edges.push(
                HalfEdge::partial(self.stores, self.surface)
                    .as_line_segment_from_points(points)
                    .build(),
            );
        }

        self
    }

    /// Close the [`Cycle`] with a line segment
    ///
    /// Builds a line segment from the last and first vertex, closing the cycle.
    pub fn close_with_line_segment(mut self) -> Self {
        let first = self.half_edges.first();
        let last = self.half_edges.last();

        if let [Some([first, _]), Some([_, last])] = [first, last]
            .map(|option| option.map(|half_edge| half_edge.vertices()))
        {
            let vertices =
                [last, first].map(|vertex| vertex.surface_form().position());
            self.half_edges.push(
                HalfEdge::partial(self.stores, self.surface)
                    .as_line_segment_from_points(vertices)
                    .build(),
            );
        }

        self
    }

    /// Finish building the [`Cycle`]
    pub fn build(self) -> Cycle {
        Cycle::new(self.surface, self.half_edges)
    }
}
