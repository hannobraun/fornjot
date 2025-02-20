use std::ops::Deref;

use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, update::UpdateHalfEdge},
    storage::Handle,
    topology::{
        Curve, Cycle, Face, HalfEdge, IsObject, Region, Shell, Sketch, Solid,
    },
};

use super::ReplaceOutput;

/// Replace a [`Curve`] in the referenced object graph
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait ReplaceCurve: IsObject + Sized {
    /// Replace the curve
    #[must_use]
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject>;
}

impl ReplaceCurve for HalfEdge {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        if original.id() == self.curve().id() {
            ReplaceOutput::Updated(self.update_curve(|_, _| replacement, core))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurve for Cycle {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut half_edges = Vec::new();
        for original_half_edge in self.half_edges() {
            let half_edge = original_half_edge.replace_curve(
                original,
                replacement.clone(),
                core,
            );
            replacement_happened |= half_edge.was_updated();
            half_edges.push(
                half_edge
                    .map_updated(|updated| {
                        updated
                            .insert(core)
                            .derive_from(original_half_edge, core)
                    })
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Cycle::new(half_edges))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurve for Region {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let exterior =
            self.exterior()
                .replace_curve(original, replacement.clone(), core);
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for original_cycle in self.interiors() {
            let cycle = original_cycle.replace_curve(
                original,
                replacement.clone(),
                core,
            );
            replacement_happened |= cycle.was_updated();
            interiors.push(
                cycle
                    .map_updated(|updated| {
                        updated.insert(core).derive_from(original_cycle, core)
                    })
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Region::new(
                exterior
                    .map_updated(|updated| {
                        updated.insert(core).derive_from(self.exterior(), core)
                    })
                    .into_inner(),
                interiors,
            ))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurve for Sketch {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for original_region in self.regions() {
            let region = original_region.replace_curve(
                original,
                replacement.clone(),
                core,
            );
            replacement_happened |= region.was_updated();
            regions.push(
                region
                    .map_updated(|updated| {
                        updated.insert(core).derive_from(original_region, core)
                    })
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Sketch::new(self.surface().clone(), regions))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurve for Face {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region = self.region().replace_curve(original, replacement, core);

        if region.was_updated() {
            ReplaceOutput::Updated(Face::new(
                self.surface().clone(),
                region
                    .map_updated(|updated| {
                        updated.insert(core).derive_from(self.region(), core)
                    })
                    .into_inner(),
            ))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurve for Shell {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for original_face in self.faces() {
            let face = original_face.replace_curve(
                original,
                replacement.clone(),
                core,
            );
            replacement_happened |= face.was_updated();
            faces.push(
                face.map_updated(|updated| {
                    updated.insert(core).derive_from(original_face, core)
                })
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

impl ReplaceCurve for Solid {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for original_shell in self.shells() {
            let shell = original_shell.replace_curve(
                original,
                replacement.clone(),
                core,
            );
            replacement_happened |= shell.was_updated();
            shells.push(
                shell
                    .map_updated(|updated| {
                        updated.insert(core).derive_from(original_shell, core)
                    })
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

impl ReplaceCurve for Handle<HalfEdge> {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curve(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurve for Handle<Cycle> {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curve(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurve for Handle<Region> {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curve(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurve for Handle<Sketch> {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curve(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurve for Handle<Face> {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curve(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurve for Handle<Shell> {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curve(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurve for Handle<Solid> {
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curve(original, replacement, core)
            .map_original(|_| self.clone())
    }
}
