use std::ops::Deref;

use crate::{
    Core,
    operations::{
        derive::DeriveFrom, insert::Insert, selector::Selector,
        update::UpdateHalfEdge,
    },
    storage::Handle,
    topology::{
        Curve, Cycle, Face, HalfEdge, IsObject, ObjectSet, Region, Shell,
        Sketch, Solid,
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

/// Replace a [`Curve`] with flexible selectors
///
/// This trait provides a more flexible interface for replacing curves, allowing
/// objects to be selected using the `Selector` trait.
pub trait ReplaceCurveWithSelector: IsObject + Sized {
    /// Replace curves selected by the given selector
    #[must_use]
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
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

impl ReplaceCurveWithSelector for HalfEdge {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // For HalfEdge, there's only one curve, so the selector selects from a single-item set
        use crate::topology::ObjectSet;
        let curve_set = ObjectSet::new([self.curve().clone()]);
        let selected_handles: Vec<_> = selector.select(&curve_set).collect();

        if let Some(curve_handle) = selected_handles.first() {
            self.replace_curve(curve_handle, replacement, core)
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurveWithSelector for Cycle {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect all curves from half edges
        let all_curves: Vec<_> = self
            .half_edges()
            .iter()
            .map(|he| he.curve().clone())
            .collect();
        let curve_set = ObjectSet::new(all_curves);
        let selected_handles: Vec<_> = selector.select(&curve_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for curve_handle in selected_handles {
            match result.replace_curve(curve_handle, replacement.clone(), core)
            {
                ReplaceOutput::Updated(updated) => {
                    result = updated;
                    replacement_happened = true;
                }
                ReplaceOutput::Original(_) => {}
            }
        }

        if replacement_happened {
            ReplaceOutput::Updated(result)
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurveWithSelector for Region {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect all curves from exterior and interiors
        let mut all_curves: Vec<_> = self
            .exterior()
            .half_edges()
            .iter()
            .map(|he| he.curve().clone())
            .collect();

        for interior in self.interiors() {
            all_curves.extend(
                interior.half_edges().iter().map(|he| he.curve().clone()),
            );
        }

        let curve_set = ObjectSet::new(all_curves);
        let selected_handles: Vec<_> = selector.select(&curve_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for curve_handle in selected_handles {
            match result.replace_curve(curve_handle, replacement.clone(), core)
            {
                ReplaceOutput::Updated(updated) => {
                    result = updated;
                    replacement_happened = true;
                }
                ReplaceOutput::Original(_) => {}
            }
        }

        if replacement_happened {
            ReplaceOutput::Updated(result)
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurveWithSelector for Sketch {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect curves from all regions
        let mut all_curves: Vec<_> = Vec::new();
        for region in self.regions() {
            all_curves.extend(
                region
                    .exterior()
                    .half_edges()
                    .iter()
                    .map(|he| he.curve().clone()),
            );

            for interior in region.interiors() {
                all_curves.extend(
                    interior.half_edges().iter().map(|he| he.curve().clone()),
                );
            }
        }

        let curve_set = ObjectSet::new(all_curves);
        let selected_handles: Vec<_> = selector.select(&curve_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for curve_handle in selected_handles {
            match result.replace_curve(curve_handle, replacement.clone(), core)
            {
                ReplaceOutput::Updated(updated) => {
                    result = updated;
                    replacement_happened = true;
                }
                ReplaceOutput::Original(_) => {}
            }
        }

        if replacement_happened {
            ReplaceOutput::Updated(result)
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurveWithSelector for Face {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region_result =
            self.region().replace_curves(selector, replacement, core);

        match region_result {
            ReplaceOutput::Updated(updated_region) => {
                ReplaceOutput::Updated(Face::new(
                    self.surface().clone(),
                    updated_region
                        .insert(core)
                        .derive_from(self.region(), core),
                ))
            }
            ReplaceOutput::Original(_) => ReplaceOutput::Original(self.clone()),
        }
    }
}

impl ReplaceCurveWithSelector for Shell {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect curves from all faces
        let mut all_curves: Vec<_> = Vec::new();
        for face in self.faces() {
            let region = face.region();
            all_curves.extend(
                region
                    .exterior()
                    .half_edges()
                    .iter()
                    .map(|he| he.curve().clone()),
            );

            for interior in region.interiors() {
                all_curves.extend(
                    interior.half_edges().iter().map(|he| he.curve().clone()),
                );
            }
        }

        let curve_set = ObjectSet::new(all_curves);
        let selected_handles: Vec<_> = selector.select(&curve_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for curve_handle in selected_handles {
            match result.replace_curve(curve_handle, replacement.clone(), core)
            {
                ReplaceOutput::Updated(updated) => {
                    result = updated;
                    replacement_happened = true;
                }
                ReplaceOutput::Original(_) => {}
            }
        }

        if replacement_happened {
            ReplaceOutput::Updated(result)
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceCurveWithSelector for Solid {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect curves from all shells
        let mut all_curves: Vec<_> = Vec::new();
        for shell in self.shells() {
            for face in shell.faces() {
                let region = face.region();
                all_curves.extend(
                    region
                        .exterior()
                        .half_edges()
                        .iter()
                        .map(|he| he.curve().clone()),
                );

                for interior in region.interiors() {
                    all_curves.extend(
                        interior
                            .half_edges()
                            .iter()
                            .map(|he| he.curve().clone()),
                    );
                }
            }
        }

        let curve_set = ObjectSet::new(all_curves);
        let selected_handles: Vec<_> = selector.select(&curve_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for curve_handle in selected_handles {
            match result.replace_curve(curve_handle, replacement.clone(), core)
            {
                ReplaceOutput::Updated(updated) => {
                    result = updated;
                    replacement_happened = true;
                }
                ReplaceOutput::Original(_) => {}
            }
        }

        if replacement_happened {
            ReplaceOutput::Updated(result)
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

// Handle implementations
impl ReplaceCurveWithSelector for Handle<HalfEdge> {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curves(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurveWithSelector for Handle<Cycle> {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curves(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurveWithSelector for Handle<Region> {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curves(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurveWithSelector for Handle<Sketch> {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curves(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurveWithSelector for Handle<Face> {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curves(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurveWithSelector for Handle<Shell> {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curves(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceCurveWithSelector for Handle<Solid> {
    fn replace_curves(
        &self,
        selector: impl Selector<Curve>,
        replacement: Handle<Curve>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_curves(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}
