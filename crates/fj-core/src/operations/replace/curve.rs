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
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject>;
}

impl ReplaceCurve for Handle<HalfEdge> {
    type BareObject = HalfEdge;

    fn replace_curve(
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        if original.id() == self.curve().id() {
            ReplaceOutput::Updated(
                self.update_curve(|_| replacement).insert(services),
            )
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceCurve for Handle<Cycle> {
    type BareObject = Cycle;

    fn replace_curve(
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut half_edges = Vec::new();
        for half_edge in self.half_edges() {
            let half_edge = half_edge.clone().replace_curve(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= half_edge.was_updated();
            half_edges.push(half_edge.into_inner());
        }

        if replacement_happened {
            ReplaceOutput::Updated(Cycle::new(half_edges).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceCurve for Handle<Region> {
    type BareObject = Region;

    fn replace_curve(
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let exterior = self.exterior().clone().replace_curve(
            original,
            replacement.clone(),
            services,
        );
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for cycle in self.interiors() {
            let cycle = cycle.clone().replace_curve(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= cycle.was_updated();
            interiors.push(cycle.into_inner());
        }

        if replacement_happened {
            ReplaceOutput::Updated(
                Region::new(exterior.into_inner(), interiors, self.color())
                    .insert(services),
            )
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceCurve for Handle<Sketch> {
    type BareObject = Sketch;

    fn replace_curve(
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for region in self.regions() {
            let region = region.clone().replace_curve(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= region.was_updated();
            regions.push(region.into_inner());
        }

        if replacement_happened {
            ReplaceOutput::Updated(Sketch::new(regions).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceCurve for Handle<Face> {
    type BareObject = Face;

    fn replace_curve(
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let region = self.region().clone().replace_curve(
            original,
            replacement,
            services,
        );

        if region.was_updated() {
            ReplaceOutput::Updated(
                Face::new(self.surface().clone(), region.into_inner())
                    .insert(services),
            )
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceCurve for Handle<Shell> {
    type BareObject = Shell;

    fn replace_curve(
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for face in self.faces() {
            let face = face.clone().replace_curve(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= face.was_updated();
            faces.push(face.into_inner());
        }

        if replacement_happened {
            ReplaceOutput::Updated(Shell::new(faces).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}

impl ReplaceCurve for Handle<Solid> {
    type BareObject = Solid;

    fn replace_curve(
        self,
        original: &Handle<Curve>,
        replacement: Handle<Curve>,
        services: &mut Services,
    ) -> ReplaceOutput<Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for shell in self.shells() {
            let shell = shell.clone().replace_curve(
                original,
                replacement.clone(),
                services,
            );
            replacement_happened |= shell.was_updated();
            shells.push(shell.into_inner());
        }

        if replacement_happened {
            ReplaceOutput::Updated(Solid::new(shells).insert(services))
        } else {
            ReplaceOutput::Original(self)
        }
    }
}
