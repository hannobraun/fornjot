use crate::{
    objects::{Cycle, HalfEdge, Objects, Surface},
    partial::{
        util::merge_options, MaybePartial, PartialHalfEdge, PartialVertex,
    },
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`Cycle`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialCycle {
    half_edges: Vec<MaybePartial<HalfEdge>>,
}

impl PartialCycle {
    /// Access the half-edges that make up the [`Cycle`]
    pub fn half_edges(&self) -> impl Iterator<Item = MaybePartial<HalfEdge>> {
        self.half_edges.clone().into_iter()
    }

    /// Access the surface that the [`Cycle`]'s [`HalfEdge`]s are defined in
    pub fn surface(&self) -> Option<Handle<Surface>> {
        self.half_edges
            .first()
            .and_then(|half_edge| half_edge.curve().surface())
    }

    /// Add the provided half-edges to the partial cycle
    ///
    /// This will merge all the surfaces of the added half-edges. All added
    /// half-edges will end up with the same merged surface.
    ///
    /// # Panics
    ///
    /// Panics, if the surfaces can't be merged.
    pub fn with_half_edges(
        mut self,
        half_edges: impl IntoIterator<Item = impl Into<MaybePartial<HalfEdge>>>,
    ) -> Self {
        let half_edges = half_edges.into_iter().map(Into::into);

        let mut surface = self.surface();
        for half_edge in half_edges {
            surface = merge_options(surface, half_edge.curve().surface());
            self.half_edges.push(half_edge);
        }

        self.with_surface(surface)
    }

    /// Update the partial cycle with the provided surface
    ///
    /// All [`HalfEdge`]s will be updated with this surface.
    pub fn with_surface(mut self, surface: Option<Handle<Surface>>) -> Self {
        if let Some(surface) = surface {
            for half_edge in &mut self.half_edges {
                *half_edge = half_edge.clone().update_partial(|half_edge| {
                    half_edge.with_surface(Some(surface.clone()))
                });
            }
        }
        self
    }

    /// Merge this partial object with another
    pub fn merge_with(self, other: Self) -> Self {
        let a_is_empty = self.half_edges.is_empty();
        let b_is_empty = other.half_edges.is_empty();
        let half_edges = match (a_is_empty, b_is_empty) {
            (true, true) => {
                panic!("Can't merge `PartialHalfEdge`, if both have half-edges")
            }
            (true, false) => self.half_edges,
            (false, true) => other.half_edges,
            (false, false) => self.half_edges, // doesn't matter which we use
        };

        Self { half_edges }
    }

    /// Build a full [`Cycle`] from the partial cycle
    pub fn build(
        mut self,
        objects: &Objects,
    ) -> Result<Handle<Cycle>, ValidationError> {
        let half_edges = {
            let last_vertex = {
                let last_half_edge = self
                    .half_edges
                    .last_mut()
                    .expect("Can't build cycle without any half-edges");

                let surface_vertex =
                    last_half_edge.front().surface_form().into_full(objects)?;

                *last_half_edge = last_half_edge.clone().merge_with(
                    PartialHalfEdge::default().with_front_vertex(Some(
                        PartialVertex::default()
                            .with_surface_form(surface_vertex.clone()),
                    )),
                );

                surface_vertex
            };

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

                            half_edge.with_back_vertex(Some(back))
                        })
                        .into_full(objects)?;

                    let front = half_edge.front().surface_form().clone();
                    half_edges.push(half_edge);

                    Ok((half_edges, front))
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
            half_edges: cycle.half_edges().cloned().map(Into::into).collect(),
        }
    }
}
