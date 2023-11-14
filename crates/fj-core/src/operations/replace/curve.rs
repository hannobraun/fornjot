use crate::{
    objects::{Curve, Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid},
    operations::{insert::Insert, update::UpdateHalfEdge},
    services::Services,
    storage::Handle,
};

use super::ReplaceOutput;

/// Replace a [`Curve`] in the referenced object graph
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait ReplaceCurve: Sized {
    /// The bare object type that this trait is implemented for
    type BareObject;

    /// Replace the curve
    #[must_use]
    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject>;
}

impl ReplaceCurve for Handle<HalfEdge> {
    type BareObject = HalfEdge;

    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        _: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        if original.id() == self.curve().id() {
            ReplaceOutput::Updated(self.update_curve(|_| replacement))
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurve for Handle<Cycle> {
    type BareObject = Cycle;

    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut half_edges = Vec::new();
        for half_edge in self.half_edges() {
            let half_edge = half_edge.replace_curve(
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

impl ReplaceCurve for Handle<Region> {
    type BareObject = Region;

    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let exterior = self.exterior().replace_curve(
            original,
            replacement.clone(),
            services,
        );
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for cycle in self.interiors() {
            let cycle =
                cycle.replace_curve(original, replacement.clone(), services);
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

impl ReplaceCurve for Handle<Sketch> {
    type BareObject = Sketch;

    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for region in self.regions() {
            let region =
                region.replace_curve(original, replacement.clone(), services);
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

impl ReplaceCurve for Handle<Face> {
    type BareObject = Face;

    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region =
            self.region().replace_curve(original, replacement, services);

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

impl ReplaceCurve for Handle<Shell> {
    type BareObject = Shell;

    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for face in self.faces() {
            let face =
                face.replace_curve(original, replacement.clone(), services);
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

impl ReplaceCurve for Handle<Solid> {
    type BareObject = Solid;

    fn replace_curve(
        &self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for shell in self.shells() {
            let shell =
                shell.replace_curve(original, replacement.clone(), services);
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
