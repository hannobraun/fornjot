use fj_math::Point;

use crate::{
    objects::{Curve, HalfEdge, Surface, SurfaceVertex, Vertex},
    partial::{HasPartial, MaybePartial, PartialCycle},
    storage::Handle,
};

use super::{CurveBuilder, HalfEdgeBuilder};

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder {
    /// Update the partial cycle with a polygonal chain from the provided points
    fn with_poly_chain(
        self,
        vertices: impl IntoIterator<Item = impl Into<MaybePartial<SurfaceVertex>>>,
    ) -> Self;

    /// Update the partial cycle with a polygonal chain from the provided points
    fn with_poly_chain_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;

    /// Update the partial cycle by closing it with a line segment
    ///
    /// Builds a line segment from the last and first vertex, closing the cycle.
    fn close_with_line_segment(self) -> Self;
}

impl CycleBuilder for PartialCycle {
    fn with_poly_chain(
        self,
        vertices: impl IntoIterator<Item = impl Into<MaybePartial<SurfaceVertex>>>,
    ) -> Self {
        let vertices = vertices.into_iter().map(Into::into);

        let iter = self
            .half_edges()
            .last()
            .map(|half_edge| {
                let [_, last] = half_edge.vertices();
                last.surface_form()
            })
            .into_iter()
            .chain(vertices);

        let mut previous: Option<MaybePartial<SurfaceVertex>> = None;

        let mut half_edges = Vec::new();
        for vertex_next in iter {
            if let Some(vertex_prev) = previous {
                let surface = self
                    .surface()
                    .clone()
                    .expect("Need surface to extend cycle with poly-chain");

                let position_prev = vertex_prev
                    .position()
                    .expect("Need surface position to extend cycle");
                let position_next = vertex_next
                    .position()
                    .expect("Need surface position to extend cycle");

                let from = vertex_prev.update_partial(|partial| {
                    partial.with_surface(Some(surface.clone()))
                });
                let to = vertex_next.update_partial(|partial| {
                    partial.with_surface(Some(surface.clone()))
                });

                previous = Some(to.clone());

                let curve = Curve::partial()
                    .with_surface(Some(surface.clone()))
                    .update_as_line_from_points([position_prev, position_next]);

                let [from, to] =
                    [(0., from), (1., to)].map(|(position, surface_form)| {
                        Vertex::partial()
                            .with_curve(Some(curve.clone()))
                            .with_position(Some([position]))
                            .with_surface_form(Some(surface_form))
                    });

                half_edges.push(
                    HalfEdge::partial()
                        .with_curve(Some(curve))
                        .with_vertices(Some([from, to])),
                );

                continue;
            }

            previous = Some(vertex_next);
        }

        self.with_half_edges(half_edges)
    }

    fn with_poly_chain_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        self.with_poly_chain(points.into_iter().map(|position| {
            SurfaceVertex::partial()
                .with_surface(Some(surface.clone()))
                .with_position(Some(position))
        }))
    }

    fn close_with_line_segment(self) -> Self {
        let first = self.half_edges().next();
        let last = self.half_edges().last();

        let vertices = [first, last]
            .map(|option| option.map(|half_edge| half_edge.vertices()));

        let [Some([first, _]), Some([_, last])] = vertices else {
            return self;
        };

        let vertices = [last, first].map(|vertex| {
            vertex
                .surface_form()
                .position()
                .expect("Need surface position to close cycle")
        });
        let surface = self.surface().expect("Need surface to close cycle");

        self.with_half_edges(Some(
            HalfEdge::partial()
                .update_as_line_segment_from_points(surface, vertices),
        ))
    }
}
