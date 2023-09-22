use crate::{
    objects::{Edge, Face, Shell, Surface},
    storage::Handle,
};

/// Access all edges referenced by the object and the surface they're on
pub trait AllEdgesWithSurface {
    /// Access all edges referenced by the object and the surface they're on
    fn all_edges_with_surface(
        &self,
        result: &mut Vec<(Handle<Edge>, Handle<Surface>)>,
    );
}

impl AllEdgesWithSurface for Face {
    fn all_edges_with_surface(
        &self,
        result: &mut Vec<(Handle<Edge>, Handle<Surface>)>,
    ) {
        for cycle in self.region().all_cycles() {
            result.extend(
                cycle
                    .edges()
                    .iter()
                    .cloned()
                    .map(|edge| (edge, self.surface().clone())),
            );
        }
    }
}

impl AllEdgesWithSurface for Shell {
    fn all_edges_with_surface(
        &self,
        result: &mut Vec<(Handle<Edge>, Handle<Surface>)>,
    ) {
        for face in self.faces() {
            face.all_edges_with_surface(result);
        }
    }
}
