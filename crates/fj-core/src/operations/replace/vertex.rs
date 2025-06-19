use std::ops::Deref;

use crate::{
    Core,
    operations::{
        derive::DeriveFrom, insert::Insert, selector::Selector,
        update::UpdateHalfEdge,
    },
    storage::Handle,
    topology::{
        Cycle, Face, HalfEdge, IsObject, ObjectSet, Region, Shell, Sketch,
        Solid, Vertex,
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

/// Replace a [`Vertex`] with flexible selectors
///
/// This trait provides a more flexible interface for replacing vertices, allowing
/// objects to be selected using the `Selector` trait.
pub trait ReplaceVertexWithSelector: IsObject + Sized {
    /// Replace vertices selected by the given selector
    #[must_use]
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
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

impl ReplaceVertexWithSelector for HalfEdge {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // For HalfEdge, there's only one start vertex, so the selector selects from a single-item set
        let vertex_set = ObjectSet::new([self.start_vertex().clone()]);
        let selected_handles: Vec<_> = selector.select(&vertex_set).collect();

        if let Some(vertex_handle) = selected_handles.first() {
            self.replace_vertex(vertex_handle, replacement, core)
        } else {
            ReplaceOutput::Original(self.clone())
        }
    }
}

impl ReplaceVertexWithSelector for Cycle {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect all vertices from half edges
        let all_vertices: Vec<_> = self
            .half_edges()
            .iter()
            .map(|he| he.start_vertex().clone())
            .collect();
        let vertex_set = ObjectSet::new(all_vertices);
        let selected_handles: Vec<_> = selector.select(&vertex_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for vertex_handle in selected_handles {
            match result.replace_vertex(
                vertex_handle,
                replacement.clone(),
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

impl ReplaceVertexWithSelector for Region {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect all vertices from exterior and interiors
        let mut all_vertices: Vec<_> = self
            .exterior()
            .half_edges()
            .iter()
            .map(|he| he.start_vertex().clone())
            .collect();

        for interior in self.interiors() {
            all_vertices.extend(
                interior
                    .half_edges()
                    .iter()
                    .map(|he| he.start_vertex().clone()),
            );
        }

        let vertex_set = ObjectSet::new(all_vertices);
        let selected_handles: Vec<_> = selector.select(&vertex_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for vertex_handle in selected_handles {
            match result.replace_vertex(
                vertex_handle,
                replacement.clone(),
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

impl ReplaceVertexWithSelector for Sketch {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect vertices from all regions
        let mut all_vertices: Vec<_> = Vec::new();
        for region in self.regions() {
            all_vertices.extend(
                region
                    .exterior()
                    .half_edges()
                    .iter()
                    .map(|he| he.start_vertex().clone()),
            );

            for interior in region.interiors() {
                all_vertices.extend(
                    interior
                        .half_edges()
                        .iter()
                        .map(|he| he.start_vertex().clone()),
                );
            }
        }

        let vertex_set = ObjectSet::new(all_vertices);
        let selected_handles: Vec<_> = selector.select(&vertex_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for vertex_handle in selected_handles {
            match result.replace_vertex(
                vertex_handle,
                replacement.clone(),
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

impl ReplaceVertexWithSelector for Face {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        let region_result =
            self.region().replace_vertices(selector, replacement, core);

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

impl ReplaceVertexWithSelector for Shell {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect vertices from all faces
        let mut all_vertices: Vec<_> = Vec::new();
        for face in self.faces() {
            let region = face.region();
            all_vertices.extend(
                region
                    .exterior()
                    .half_edges()
                    .iter()
                    .map(|he| he.start_vertex().clone()),
            );

            for interior in region.interiors() {
                all_vertices.extend(
                    interior
                        .half_edges()
                        .iter()
                        .map(|he| he.start_vertex().clone()),
                );
            }
        }

        let vertex_set = ObjectSet::new(all_vertices);
        let selected_handles: Vec<_> = selector.select(&vertex_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for vertex_handle in selected_handles {
            match result.replace_vertex(
                vertex_handle,
                replacement.clone(),
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

impl ReplaceVertexWithSelector for Solid {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        // Collect vertices from all shells
        let mut all_vertices: Vec<_> = Vec::new();
        for shell in self.shells() {
            for face in shell.faces() {
                let region = face.region();
                all_vertices.extend(
                    region
                        .exterior()
                        .half_edges()
                        .iter()
                        .map(|he| he.start_vertex().clone()),
                );

                for interior in region.interiors() {
                    all_vertices.extend(
                        interior
                            .half_edges()
                            .iter()
                            .map(|he| he.start_vertex().clone()),
                    );
                }
            }
        }

        let vertex_set = ObjectSet::new(all_vertices);
        let selected_handles: Vec<_> = selector.select(&vertex_set).collect();

        if selected_handles.is_empty() {
            return ReplaceOutput::Original(self.clone());
        }

        let mut result = self.clone();
        let mut replacement_happened = false;

        for vertex_handle in selected_handles {
            match result.replace_vertex(
                vertex_handle,
                replacement.clone(),
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
impl ReplaceVertexWithSelector for Handle<HalfEdge> {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertices(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertexWithSelector for Handle<Cycle> {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertices(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertexWithSelector for Handle<Region> {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertices(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertexWithSelector for Handle<Sketch> {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertices(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertexWithSelector for Handle<Face> {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertices(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertexWithSelector for Handle<Shell> {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertices(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}

impl ReplaceVertexWithSelector for Handle<Solid> {
    fn replace_vertices(
        &self,
        selector: impl Selector<Vertex>,
        replacement: Handle<Vertex>,
        core: &mut Core,
    ) -> ReplaceOutput<Self, Self::BareObject> {
        self.deref()
            .replace_vertices(selector, replacement, core)
            .map_original(|_| self.clone())
    }
}
