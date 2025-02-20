use std::ops::Deref;

use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, update::UpdateHalfEdge},
    storage::Handle,
    topology::{
        Cycle, Face, HalfEdge, IsObject, Region, Shell, Sketch, Solid, Vertex,
    },
};

use super::ReplaceOutput;

/// Replace a [`Vertex`] in the referenced object graph
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait ReplaceVertex: IsObject + Sized {
    /// Replace the vertex
    #[must_use]
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject>;
}

impl ReplaceVertex for HalfEdge {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        if original.id() == self.start_vertex().id() {
            ReplaceOutput::Updated(
                self.update_start_vertex(|_, _| replacement, core),
            )
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceVertex for Cycle {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut half_edges = Vec::new();
        for original_half_edge in self.half_edges() {
            let half_edge = original_half_edge.replace_vertex(
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

impl ReplaceVertex for Region {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let exterior =
            self.exterior()
                .replace_vertex(original, replacement.clone(), core);
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for original_cycle in self.interiors() {
            let cycle = original_cycle.replace_vertex(
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

impl ReplaceVertex for Sketch {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for original_region in self.regions() {
            let region = original_region.replace_vertex(
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

impl ReplaceVertex for Face {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region = self.region().replace_vertex(original, replacement, core);

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

impl ReplaceVertex for Shell {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for original_face in self.faces() {
            let face = original_face.replace_vertex(
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

impl ReplaceVertex for Solid {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for original_shell in self.shells() {
            let shell = original_shell.replace_vertex(
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

impl ReplaceVertex for Handle<HalfEdge> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Cycle> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Region> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Sketch> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for original_region in self.regions() {
            let region = original_region.replace_vertex(
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

impl ReplaceVertex for Handle<Face> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Shell> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Solid> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, core)
            .map_original(|_| self.clone())
    }
}
