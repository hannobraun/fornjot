use crate::{
    objects::{Cycle, HalfEdge, Objects, Surface},
    partial::{MaybePartial, MergeWith},
    partial2::Partial,
    services::Service,
};

/// A partial [`Cycle`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialCycle {
    /// The half-edges that make up the [`Cycle`]
    pub half_edges: Vec<Partial<HalfEdge>>,
}

impl PartialCycle {
    /// Access the surface that the [`Cycle`]'s [`HalfEdge`]s are defined in
    pub fn surface(&self) -> Option<Partial<Surface>> {
        self.half_edges
            .first()
            .map(|half_edge| half_edge.read().curve().read().surface.clone())
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
        half_edges: impl IntoIterator<Item = Partial<HalfEdge>>,
    ) -> Self {
        let half_edges = half_edges.into_iter().map(Into::into);

        for half_edge in half_edges {
            self.half_edges.push(half_edge);
        }

        self
    }

    /// Build a full [`Cycle`] from the partial cycle
    pub fn build(self, objects: &mut Service<Objects>) -> Cycle {
        let mut half_edges = Vec::new();
        for half_edge in self.half_edges {
            let half_edge = half_edge.build(objects);
            half_edges.push(half_edge);
        }

        Cycle::new(half_edges)
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
            half_edges: cycle
                .half_edges()
                .cloned()
                .map(Partial::from_full_entry_point)
                .collect(),
        }
    }
}

impl MaybePartial<Cycle> {
    /// Access the surface
    pub fn surface(&self) -> Option<Partial<Surface>> {
        match self {
            Self::Full(full) => {
                Some(Partial::from_full_entry_point(full.surface().clone()))
            }
            Self::Partial(partial) => partial.surface(),
        }
    }
}
