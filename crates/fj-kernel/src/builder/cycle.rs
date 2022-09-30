use fj_math::Point;

use crate::{
    objects::{Curve, Cycle, HalfEdge, Surface, SurfaceVertex, Vertex},
    partial::HasPartial,
    stores::{Handle, Stores},
};

/// API for building a [`Cycle`]
///
/// Also see [`Cycle::builder`].
pub struct CycleBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Cycle`] is defined in
    pub surface: Handle<Surface>,

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
        let iter = self
            .half_edges
            .last()
            .map(|half_edge| {
                let [_, last] = half_edge.vertices();

                let vertex = last.surface_form().clone();
                let position = last.surface_form().position();

                (position, Some(vertex))
            })
            .into_iter()
            .chain(points.into_iter().map(|point| (point.into(), None)));

        let mut previous: Option<(Point<2>, Option<SurfaceVertex>)> = None;

        for (position, vertex) in iter {
            if let Some((previous_position, previous_vertex)) = previous {
                let from = previous_vertex.unwrap_or_else(|| {
                    SurfaceVertex::partial()
                        .with_surface(self.surface.clone())
                        .with_position(previous_position)
                        .build(self.stores)
                });
                let to = vertex.unwrap_or_else(|| {
                    SurfaceVertex::partial()
                        .with_surface(self.surface.clone())
                        .with_position(position)
                        .build(self.stores)
                });

                previous = Some((position, Some(to.clone())));

                let curve = Curve::partial()
                    .with_surface(self.surface.clone())
                    .as_line_from_points([previous_position, position])
                    .build(self.stores);

                let [from, to] =
                    [(0., from), (1., to)].map(|(position, surface_form)| {
                        Vertex::partial()
                            .with_curve(curve.clone())
                            .with_position([position])
                            .with_surface_form(surface_form)
                            .build(self.stores)
                    });

                self.half_edges.push(
                    HalfEdge::partial()
                        .with_curve(curve)
                        .with_vertices([from, to])
                        .build(self.stores),
                );

                continue;
            }

            previous = Some((position, vertex));
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
                HalfEdge::partial()
                    .as_line_segment_from_points(self.surface.clone(), vertices)
                    .build(self.stores),
            );
        }

        self
    }

    /// Finish building the [`Cycle`]
    pub fn build(self) -> Cycle {
        Cycle::new(self.surface, self.half_edges)
    }
}
