use fj_math::Point;

use crate::{
    objects::{Curve, Cycle, HalfEdge, Stores, Surface, SurfaceVertex, Vertex},
    partial::{HasPartial, MaybePartial},
    storage::Handle,
};

/// A partial [`Cycle`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialCycle {
    /// The surface that the [`Cycle`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The half-edges that make up the [`Cycle`]
    pub half_edges: Vec<MaybePartial<HalfEdge>>,
}

impl PartialCycle {
    /// Update the partial cycle with the given surface
    pub fn with_surface(mut self, surface: Option<Handle<Surface>>) -> Self {
        if let Some(surface) = surface {
            self.surface = Some(surface);
        }
        self
    }

    /// Update the partial cycle with the given half-edges
    pub fn with_half_edges(
        mut self,
        half_edge: impl IntoIterator<Item = impl Into<MaybePartial<HalfEdge>>>,
    ) -> Self {
        self.half_edges
            .extend(half_edge.into_iter().map(Into::into));
        self
    }

    /// Update the partial cycle with a polygonal chain from the provided points
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
                let surface = self
                    .surface
                    .clone()
                    .expect("Need surface to extend cycle with poly-chain");

                let from = previous_vertex.unwrap_or_else(|| {
                    SurfaceVertex::partial()
                        .with_surface(Some(surface.clone()))
                        .with_position(Some(previous_position))
                        .into()
                });
                let to = vertex.unwrap_or_else(|| {
                    SurfaceVertex::partial()
                        .with_surface(Some(surface.clone()))
                        .with_position(Some(position))
                        .into()
                });

                previous = Some((position, Some(to.clone())));

                let curve = Handle::<Curve>::partial()
                    .with_surface(Some(surface.clone()))
                    .as_line_from_points([previous_position, position]);

                let [from, to] =
                    [(0., from), (1., to)].map(|(position, surface_form)| {
                        Vertex::partial()
                            .with_curve(Some(curve.clone()))
                            .with_position(Some([position]))
                            .with_surface_form(Some(surface_form))
                    });

                self.half_edges.push(
                    HalfEdge::partial()
                        .with_curve(Some(curve))
                        .with_vertices(Some([from, to]))
                        .into(),
                );

                continue;
            }

            previous = Some((position, vertex));
        }

        self
    }

    /// Update the partial cycle by closing it with a line segment
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
            let surface =
                self.surface.clone().expect("Need surface to close cycle");

            self.half_edges.push(
                HalfEdge::partial()
                    .with_surface(Some(surface))
                    .as_line_segment_from_points(vertices)
                    .into(),
            );
        }

        self
    }

    /// Build a full [`Cycle`] from the partial cycle
    pub fn build(self, stores: &Stores) -> Cycle {
        let surface = self.surface.expect("Need surface to build `Cycle`");
        let surface_for_edges = surface.clone();
        let half_edges = self.half_edges.into_iter().map(|half_edge| {
            half_edge
                .update_partial(|half_edge| {
                    half_edge.with_surface(Some(surface_for_edges.clone()))
                })
                .into_full(stores)
        });

        Cycle::new(surface, half_edges)
    }
}

impl From<&Cycle> for PartialCycle {
    fn from(cycle: &Cycle) -> Self {
        Self {
            surface: Some(cycle.surface().clone()),
            half_edges: cycle.half_edges().cloned().map(Into::into).collect(),
        }
    }
}
