use std::ops::Deref;

use crate::{
    objects::{
        Cycle, Face, HalfEdge, IsObject, Region, Shell, Sketch, Solid, Vertex,
    },
    operations::{insert::Insert, update::UpdateHalfEdge},
    services::Services,
    storage::Handle,
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
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject>;
}

impl ReplaceVertex for HalfEdge {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        _: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        if original.id() == self.start_vertex().id() {
            ReplaceOutput::Updated(self.update_start_vertex(|_| replacement))
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
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut half_edges = Vec::new();
        for half_edge in self.half_edges() {
            let half_edge = half_edge.replace_vertex(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= half_edge.was_updated();
            half_edges.push(
                half_edge
                    .map_updated(|updated| updated.insert(services))
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
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let exterior = self.exterior().replace_vertex(
            original,
            replacement.clone(),
            services,
        );
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for cycle in self.interiors() {
            let cycle =
                cycle.replace_vertex(original, replacement.clone(), services);
            replacement_happened |= cycle.was_updated();
            interiors.push(
                cycle
                    .map_updated(|updated| updated.insert(services))
                    .into_inner(),
            );
        }

        if replacement_happened {
            ReplaceOutput::Updated(Region::new(
                exterior
                    .map_updated(|updated| updated.insert(services))
                    .into_inner(),
                interiors,
                self.color(),
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
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for region in self.regions() {
            let region =
                region.replace_vertex(original, replacement.clone(), services);
            replacement_happened |= region.was_updated();
            regions.push(
                region
                    .map_updated(|updated| updated.insert(services))
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

impl ReplaceVertex for Face {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region =
            self.region()
                .replace_vertex(original, replacement, services);

        if region.was_updated() {
            ReplaceOutput::Updated(Face::new(
                self.surface().clone(),
                region
                    .map_updated(|updated| updated.insert(services))
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
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for face in self.faces() {
            let face =
                face.replace_vertex(original, replacement.clone(), services);
            replacement_happened |= face.was_updated();
            faces.push(
                face.map_updated(|updated| updated.insert(services))
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
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for shell in self.shells() {
            let shell =
                shell.replace_vertex(original, replacement.clone(), services);
            replacement_happened |= shell.was_updated();
            shells.push(
                shell
                    .map_updated(|updated| updated.insert(services))
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
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, services)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Cycle> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, services)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Region> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, services)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Sketch> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for region in self.regions() {
            let region =
                region.replace_vertex(original, replacement.clone(), services);
            replacement_happened |= region.was_updated();
            regions.push(
                region
                    .map_updated(|updated| updated.insert(services))
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

impl ReplaceVertex for Handle<Face> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, services)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Shell> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, services)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertex for Handle<Solid> {
    fn replace_vertex(
        &self,
        original: &Handle<Vertex>,
        replacement: Handle<Vertex>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertex(original, replacement, services)
            .map_original(|_| self.clone())
    }
}
