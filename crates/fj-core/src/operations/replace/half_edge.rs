use std::ops::Deref;

use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, selector::Selector},
    storage::Handle,
    topology::{
        Cycle, Face, HalfEdge, IsObject, ObjectSet, Region, Shell, Sketch,
        Solid,
    },
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
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject>;
}

/// Replace a [`HalfEdge`] with flexible selectors
///
/// This trait provides a more flexible interface for replacing half-edges, allowing
/// objects to be selected using the `Selector` trait.
pub trait ReplaceHalfEdgeWithSelector: IsObject + Sized {
    /// Replace half-edges selected by the given selector
    #[must_use]
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject>;
}

impl ReplaceHalfEdge for Cycle {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        _: &mut Core,
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

impl ReplaceHalfEdgeWithSelector for Cycle {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let selected_handles: Vec<_> =
            selector.select(self.half_edges()).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for handle in selected_handles {
            match result.replace_half_edge(handle, replacements.clone(), core) {
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

impl ReplaceHalfEdge for Region {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let exterior = self.exterior().replace_half_edge(
            original,
            replacements.clone(),
            core,
        );
        replacement_happened |= exterior.was_updated();

        let mut interiors = Vec::new();
        for original_cycle in self.interiors() {
            let cycle = original_cycle.replace_half_edge(
                original,
                replacements.clone(),
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

impl ReplaceHalfEdge for Sketch {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut regions = Vec::new();
        for original_region in self.regions() {
            let region = original_region.replace_half_edge(
                original,
                replacements.clone(),
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

impl ReplaceHalfEdge for Face {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region =
            self.region()
                .replace_half_edge(original, replacements, core);

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

impl ReplaceHalfEdge for Shell {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut faces = Vec::new();
        for original_face in self.faces() {
            let face = original_face.replace_half_edge(
                original,
                replacements.clone(),
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

impl ReplaceHalfEdge for Solid {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let mut replacement_happened = false;

        let mut shells = Vec::new();
        for original_shell in self.shells() {
            let shell = original_shell.replace_half_edge(
                original,
                replacements.clone(),
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

impl ReplaceHalfEdge for Handle<Cycle> {
    fn replace_half_edge<const N: usize>(
        &self,
        original: &Handle<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
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
        core: &mut Core,
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
        core: &mut Core,
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
        core: &mut Core,
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
        core: &mut Core,
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
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edge(original, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdgeWithSelector for Region {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect all half edges from exterior and interiors
        let mut all_half_edges: Vec<_> =
            self.exterior().half_edges().iter().cloned().collect();

        for interior in self.interiors() {
            all_half_edges.extend(interior.half_edges().iter().cloned());
        }

        let half_edge_set = ObjectSet::new(all_half_edges);
        let selected_handles: Vec<_> =
            selector.select(&half_edge_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for half_edge_handle in selected_handles {
            match result.replace_half_edge(
                half_edge_handle,
                replacements.clone(),
                core,
            ) {
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

impl ReplaceHalfEdgeWithSelector for Sketch {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect half edges from all regions
        let mut all_half_edges: Vec<_> = Vec::new();
        for region in self.regions() {
            all_half_edges
                .extend(region.exterior().half_edges().iter().cloned());

            for interior in region.interiors() {
                all_half_edges.extend(interior.half_edges().iter().cloned());
            }
        }

        let half_edge_set = ObjectSet::new(all_half_edges);
        let selected_handles: Vec<_> =
            selector.select(&half_edge_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for half_edge_handle in selected_handles {
            match result.replace_half_edge(
                half_edge_handle,
                replacements.clone(),
                core,
            ) {
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

impl ReplaceHalfEdgeWithSelector for Face {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region_result =
            self.region()
                .replace_half_edges(selector, replacements, core);

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

impl ReplaceHalfEdgeWithSelector for Shell {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect half edges from all faces
        let mut all_half_edges: Vec<_> = Vec::new();
        for face in self.faces() {
            let region = face.region();
            all_half_edges
                .extend(region.exterior().half_edges().iter().cloned());

            for interior in region.interiors() {
                all_half_edges.extend(interior.half_edges().iter().cloned());
            }
        }

        let half_edge_set = ObjectSet::new(all_half_edges);
        let selected_handles: Vec<_> =
            selector.select(&half_edge_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for half_edge_handle in selected_handles {
            match result.replace_half_edge(
                half_edge_handle,
                replacements.clone(),
                core,
            ) {
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

impl ReplaceHalfEdgeWithSelector for Solid {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect half edges from all shells
        let mut all_half_edges: Vec<_> = Vec::new();
        for shell in self.shells() {
            for face in shell.faces() {
                let region = face.region();
                all_half_edges
                    .extend(region.exterior().half_edges().iter().cloned());

                for interior in region.interiors() {
                    all_half_edges
                        .extend(interior.half_edges().iter().cloned());
                }
            }
        }

        let half_edge_set = ObjectSet::new(all_half_edges);
        let selected_handles: Vec<_> =
            selector.select(&half_edge_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for half_edge_handle in selected_handles {
            match result.replace_half_edge(
                half_edge_handle,
                replacements.clone(),
                core,
            ) {
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
impl ReplaceHalfEdgeWithSelector for Handle<Cycle> {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edges(selector, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdgeWithSelector for Handle<Region> {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edges(selector, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdgeWithSelector for Handle<Sketch> {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edges(selector, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdgeWithSelector for Handle<Face> {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edges(selector, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdgeWithSelector for Handle<Shell> {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edges(selector, replacements, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceHalfEdgeWithSelector for Handle<Solid> {
    fn replace_half_edges<const N: usize>(
        &self,
        selector: impl Selector<HalfEdge>,
        replacements: [Handle<HalfEdge>; N],
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_half_edges(selector, replacements, core)
            .map_original(|_| self.clone())
    }
}
