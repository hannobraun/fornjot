use fj_math::Point;

use crate::{
    objects::{Curve, Cycle, HalfEdge, Surface, SurfaceVertex, Vertex},
    partial::{HasPartial, MaybePartial},
    stores::{Handle, Stores},
};

/// API for building a [`Cycle`]
///
/// Also see [`Cycle::builder`].
pub struct PartialCycle {
    /// The surface that the [`Cycle`] is defined in
    pub surface: Handle<Surface>,

    /// The half-edges that make up the [`Cycle`]
    pub half_edges: Vec<MaybePartial<HalfEdge>>,
}

impl PartialCycle {
    /// Build the [`Cycle`] with the given half-edge
    pub fn with_half_edges(
        mut self,
        half_edge: impl IntoIterator<Item = impl Into<MaybePartial<HalfEdge>>>,
    ) -> Self {
        self.half_edges
            .extend(half_edge.into_iter().map(Into::into));
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
                let [_, last] = half_edge.vertices().expect(
                    "Need half-edge vertices to extend cycle with poly-chain",
                );
                let last = last.surface_form().expect(
                    "Need surface vertex to extend cycle with poly-chain",
                );

                let vertex = last.clone();
                let position = last.position().expect(
                    "Need surface position to extend cycle with poly-chain",
                );

                (position, Some(vertex))
            })
            .into_iter()
            .chain(points.into_iter().map(|point| (point.into(), None)));

        let mut previous: Option<(
            Point<2>,
            Option<MaybePartial<SurfaceVertex>>,
        )> = None;

        for (position, vertex) in iter {
            if let Some((previous_position, previous_vertex)) = previous {
                let from = previous_vertex.unwrap_or_else(|| {
                    SurfaceVertex::partial()
                        .with_surface(self.surface.clone())
                        .with_position(previous_position)
                        .into()
                });
                let to = vertex.unwrap_or_else(|| {
                    SurfaceVertex::partial()
                        .with_surface(self.surface.clone())
                        .with_position(position)
                        .into()
                });

                previous = Some((position, Some(to.clone())));

                let curve = Curve::partial()
                    .with_surface(self.surface.clone())
                    .as_line_from_points([previous_position, position]);

                let [from, to] =
                    [(0., from), (1., to)].map(|(position, surface_form)| {
                        Vertex::partial()
                            .with_curve(curve.clone())
                            .with_position([position])
                            .with_surface_form(surface_form)
                    });

                self.half_edges.push(
                    HalfEdge::partial()
                        .with_curve(curve)
                        .with_vertices([from, to])
                        .into(),
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

        let vertices = [first, last].map(|option| {
            option.map(|half_edge| {
                half_edge.vertices().expect("Need vertices to close cycle")
            })
        });

        if let [Some([first, _]), Some([_, last])] = vertices {
            let vertices = [last, first].map(|vertex| {
                vertex
                    .surface_form()
                    .expect("Need surface vertex to close cycle")
                    .position()
                    .expect("Need surface position to close cycle")
            });
            self.half_edges.push(
                HalfEdge::partial()
                    .as_line_segment_from_points(self.surface.clone(), vertices)
                    .into(),
            );
        }

        self
    }

    /// Finish building the [`Cycle`]
    pub fn build(self, stores: &Stores) -> Cycle {
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|half_edge| half_edge.into_full(stores));

        Cycle::new(self.surface, half_edges)
    }
}
