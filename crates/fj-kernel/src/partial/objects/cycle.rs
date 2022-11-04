use crate::{
    objects::{Cycle, HalfEdge, Objects, Surface},
    partial::MaybePartial,
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
