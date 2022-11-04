use fj_math::Point;

use crate::{
    builder::CurveBuilder,
    objects::{
        Curve, Cycle, HalfEdge, Objects, Surface, SurfaceVertex, Vertex,
    },
    partial::{HasPartial, MaybePartial},
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`Cycle`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialCycle {
    surface: Option<Handle<Surface>>,
    half_edges: Vec<MaybePartial<HalfEdge>>,
}

impl PartialCycle {
    /// Access the surface that the [`Cycle`] is defined in
    pub fn surface(&self) -> Option<Handle<Surface>> {
        self.surface.clone()
    }

    /// Access the half-edges that make up the [`Cycle`]
    pub fn half_edges(&self) -> impl Iterator<Item = MaybePartial<HalfEdge>> {
        self.half_edges.clone().into_iter()
    }

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
        half_edges: impl IntoIterator<Item = impl Into<MaybePartial<HalfEdge>>>,
    ) -> Self {
        self.half_edges
            .extend(half_edges.into_iter().map(Into::into));
        self
    }

    /// Update the partial cycle with a polygonal chain from the provided points
    pub fn with_poly_chain(
        mut self,
        vertices: impl IntoIterator<Item = MaybePartial<SurfaceVertex>>,
    ) -> Self {
        let iter = self
            .half_edges
            .last()
            .map(|half_edge| {
                let [_, last] = half_edge.vertices();
                last.surface_form()
            })
            .into_iter()
            .chain(vertices);

        let mut previous: Option<MaybePartial<SurfaceVertex>> = None;

        for vertex_next in iter {
            if let Some(vertex_prev) = previous {
                let surface = self
                    .surface
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

                self.half_edges.push(
                    HalfEdge::partial()
                        .with_curve(Some(curve))
                        .with_vertices(Some([from, to]))
                        .into(),
                );

                continue;
            }

            previous = Some(vertex_next);
        }

        self
    }

    /// Update the partial cycle with a polygonal chain from the provided points
    pub fn with_poly_chain_from_points(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        self.with_poly_chain(points.into_iter().map(|position| {
            SurfaceVertex::partial()
                .with_position(Some(position))
                .into()
        }))
    }

    /// Update the partial cycle by closing it with a line segment
    ///
    /// Builds a line segment from the last and first vertex, closing the cycle.
    pub fn close_with_line_segment(mut self) -> Self {
        let first = self.half_edges.first();
        let last = self.half_edges.last();

        let vertices = [first, last]
            .map(|option| option.map(|half_edge| half_edge.vertices()));

        if let [Some([first, _]), Some([_, last])] = vertices {
            let vertices = [last, first].map(|vertex| {
                vertex
                    .surface_form()
                    .position()
                    .expect("Need surface position to close cycle")
            });
            let surface =
                self.surface.clone().expect("Need surface to close cycle");

            self.half_edges.push(
                HalfEdge::partial()
                    .with_surface(Some(surface))
                    .update_as_line_segment_from_points(vertices)
                    .into(),
            );
        }

        self
    }

    /// Build a full [`Cycle`] from the partial cycle
    pub fn build(
        mut self,
        objects: &Objects,
    ) -> Result<Handle<Cycle>, ValidationError> {
        let surface = self.surface.expect("Need surface to build `Cycle`");
        let surface_for_edges = surface.clone();
        let half_edges = {
            let last_vertex = self
                .half_edges
                .last_mut()
                .map(|half_edge| {
                    let vertex = half_edge.front();
                    (half_edge, vertex)
                })
                .map(|(half_edge, vertex)| {
                    let surface_vertex = vertex.surface_form();
                    (half_edge, vertex, surface_vertex)
                })
                .map(|(half_edge, vertex, surface_vertex)|
                    -> Result<_, ValidationError>
                {
                    let surface_vertex = surface_vertex
                        .update_partial(|surface_vertex| {
                            surface_vertex.with_surface(Some(surface.clone()))
                        })
                        .into_full(objects)?;

                    *half_edge =
                        half_edge.clone().update_partial(|half_edge| {
                            half_edge.with_front_vertex(Some(
                                vertex.update_partial(|vertex| {
                                    vertex.with_surface_form(Some(
                                        surface_vertex.clone(),
                                    ))
                                }),
                            ))
                        });

                    Ok(surface_vertex)
                })
                .transpose()?;

            let (half_edges, _) = self.half_edges.into_iter().fold(
                Ok((Vec::new(), last_vertex)),
                |result: Result<_, ValidationError>, half_edge| {
                    let (mut half_edges, previous_vertex) = result?;

                    let half_edge = half_edge
                        .update_partial(|half_edge| {
                            let [back, _] = half_edge.vertices();
                            let back = back.update_partial(|partial| {
                                partial.with_surface_form(previous_vertex)
                            });

                            half_edge
                                .with_surface(Some(surface_for_edges.clone()))
                                .with_back_vertex(Some(back))
                        })
                        .into_full(objects)?;

                    let front = half_edge.front().surface_form().clone();
                    half_edges.push(half_edge);

                    Ok((half_edges, Some(front)))
                },
            )?;

            half_edges
        };

        Ok(objects.cycles.insert(Cycle::new(half_edges))?)
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
