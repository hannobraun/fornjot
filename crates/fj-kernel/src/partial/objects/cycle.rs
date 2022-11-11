use crate::{
    builder::HalfEdgeBuilder,
    objects::{Cycle, HalfEdge, Objects, Surface},
    partial::{MaybePartial, MergeWith, PartialHalfEdge, PartialVertex},
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
            surface = surface.merge_with(half_edge.curve().surface());
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
                    half_edge.with_surface(surface.clone())
                });
            }
        }
        self
    }

    /// Build a full [`Cycle`] from the partial cycle
    pub fn build(
        mut self,
        objects: &Objects,
    ) -> Result<Cycle, ValidationError> {
        // Check that the cycle is closed. This will lead to a panic further
        // down anyway, but that panic would be super-confusing. This one should
        // be a bit more explicit on what is wrong.
        if let (Some(first), Some(last)) =
            (self.half_edges.first(), self.half_edges.last())
        {
            let [first, _] = first.vertices();
            let [_, last] = last.vertices();

            assert_eq!(
                first.surface_form().position(),
                last.surface_form().position(),
                "Attempting to build un-closed cycle"
            );
        }

        // To create a cycle, we need to make sure that all its half-edges
        // connect to each other. Let's start with all the connections between
        // the first and the last half-edge.
        let mut previous_vertex = None;
        for half_edge in &mut self.half_edges {
            let back_vertex = previous_vertex.unwrap_or_default();
            let front_vertex =
                half_edge.front().surface_form().into_full(objects)?;

            *half_edge = half_edge.clone().merge_with(
                PartialHalfEdge::default().with_vertices([
                    PartialVertex::default().with_surface_form(back_vertex),
                    PartialVertex::default()
                        .with_surface_form(front_vertex.clone()),
                ]),
            );

            previous_vertex = Some(MaybePartial::from(front_vertex));
        }

        // We're not quite done yet. We need to close the cycle, by connecting
        // the last half-edge back around to the first one.
        if let Some(half_edge) = self.half_edges.first_mut() {
            let back_vertex = previous_vertex.unwrap_or_default();

            *half_edge = half_edge.clone().merge_with(
                PartialHalfEdge::default().with_back_vertex(
                    PartialVertex::default().with_surface_form(back_vertex),
                ),
            );
        }

        // All connections made! All that's left is to build the half-edges.
        let mut half_edges = Vec::new();
        for half_edge in self.half_edges {
            let half_edge = half_edge.into_full(objects)?;
            half_edges.push(half_edge);
        }

        Ok(Cycle::new(half_edges))
    }
}

impl MergeWith for PartialCycle {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            half_edges: self.half_edges.merge_with(other.half_edges),
        }
    }
}

impl From<&Cycle> for PartialCycle {
    fn from(cycle: &Cycle) -> Self {
        Self {
            half_edges: cycle.half_edges().cloned().map(Into::into).collect(),
        }
    }
}
