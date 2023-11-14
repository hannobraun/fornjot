use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid},
    services::Services,
    storage::Handle,
};

use super::ReplaceOutput;

/// Replace a [`HalfEdge`] in the referenced object graph
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait ReplaceHalfEdge: Sized {
    /// The bare object type that this trait is implemented for
    type BareObject;

    /// Replace the half-edge
    #[must_use]
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject>;
}

impl ReplaceHalfEdge for Handle<Cycle> {
    type BareObject = Cycle;

    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        _: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        if let Some(half_edges) =
            self.half_edges().replace(original, replacements)
        {
            ReplaceOutput::Updated(Cycle::new(half_edges))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Handle<Region> {
    type BareObject = Region;

    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let exterior = self.exterior().replace_half_edge(
            original,
            replacements.clone(),
            services,
        );
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for cycle in self.interiors() {
            let cycle = cycle.replace_half_edge(
                original,
                replacements.clone(),
                services,
            );
            replacement_happened |= cycle.was_updated();
            interiors.push(cycle.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Region::new(
                exterior.into_inner(services),
                interiors,
                self.color(),
            ))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Handle<Sketch> {
    type BareObject = Sketch;

    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for region in self.regions() {
            let region = region.replace_half_edge(
                original,
                replacements.clone(),
                services,
            );
            replacement_happened |= region.was_updated();
            regions.push(region.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Sketch::new(regions))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Handle<Face> {
    type BareObject = Face;

    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let region =
            self.region()
                .replace_half_edge(original, replacements, services);

        if region.was_updated() {
            ReplaceOutput::Updated(Face::new(
                self.surface().clone(),
                region.into_inner(services),
            ))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Handle<Shell> {
    type BareObject = Shell;

    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for face in self.faces() {
            let face = face.replace_half_edge(
                original,
                replacements.clone(),
                services,
            );
            replacement_happened |= face.was_updated();
            faces.push(face.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Shell::new(faces))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceHalfEdge for Handle<Solid> {
    type BareObject = Solid;

    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for shell in self.shells() {
            let shell = shell.replace_half_edge(
                original,
                replacements.clone(),
                services,
            );
            replacement_happened |= shell.was_updated();
            shells.push(shell.into_inner(services));
        }

        if replacement_happened {
            ReplaceOutput::Updated(Solid::new(shells))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}
