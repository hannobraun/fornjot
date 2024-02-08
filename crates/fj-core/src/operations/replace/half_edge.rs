use std::ops::Deref;

use crate::{
    objects::{Cycle, Face, HalfEdge, IsObject, Region, Shell, Sketch, Solid},
    operations::insert::Insert,
    storage::Handle,
    Instance,
};

use super::ReplaceOutput;

/// Replace a [`HalfEdge`] in the referenced object graph
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait ReplaceHalfEdge: IsObject + Sized {
    /// Replace the half-edge
    #[must_use]
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject>;
}

impl ReplaceHalfEdge for Cycle {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        _: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        if let Some(half_edges) =
            self.half_edges().replace(original, replacements)
        {
            ReplaceOutput::Updated(Cycle::new(half_edges))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Region {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let exterior = self.exterior().replace_half_edge(
            original,
            replacements.clone(),
            core,
        );
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for cycle in self.interiors() {
            let cycle =
                cycle.replace_half_edge(original, replacements.clone(), core);
            replacement_happened |= cycle.was_updated();
            interiors.push(
                cycle
                    .map_updated(|updated| updated.insert(core))
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Region::new(
                exterior
                    .map_updated(|updated| updated.insert(core))
                    .into_inner(),
                interiors,
                self.color(),
            ))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Sketch {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for region in self.regions() {
            let region =
                region.replace_half_edge(original, replacements.clone(), core);
            replacement_happened |= region.was_updated();
            regions.push(
                region
                    .map_updated(|updated| updated.insert(core))
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Sketch::new(regions))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Face {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region =
            self.region()
                .replace_half_edge(original, replacements, core);

        if region.was_updated() {
            ReplaceOutput::Updated(Face::new(
                self.surface().clone(),
                region
                    .map_updated(|updated| updated.insert(core))
                    .into_inner(),
            ))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Shell {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for face in self.faces() {
            let face =
                face.replace_half_edge(original, replacements.clone(), core);
            replacement_happened |= face.was_updated();
            faces.push(
                face.map_updated(|updated| updated.insert(core))
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Shell::new(faces))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Solid {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for shell in self.shells() {
            let shell =
                shell.replace_half_edge(original, replacements.clone(), core);
            replacement_happened |= shell.was_updated();
            shells.push(
                shell
                    .map_updated(|updated| updated.insert(core))
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Solid::new(shells))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Handle<Cycle> {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edge(original, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdge for Handle<Region> {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edge(original, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdge for Handle<Sketch> {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edge(original, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdge for Handle<Face> {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edge(original, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdge for Handle<Shell> {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edge(original, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdge for Handle<Solid> {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Instance,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edge(original, replacements, core)
            .map_original(|_| self.clone())
    }
}
