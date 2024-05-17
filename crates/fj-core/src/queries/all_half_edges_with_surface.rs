use crate::{
    storage::Handle,
    topology::{Face, HalfEdge, Shell, Surface},
};

/// Access all half-edges referenced by an object, and the surface they're on
pub trait AllHalfEdgesWithSurface {
    /// Access all half-edges of the object, and the surface they're on
    fn all_half_edges_with_surface(
        &self,
    ) -> impl Iterator<Item = (Handle<HalfEdge>, Handle<Surface>)>;
}

impl AllHalfEdgesWithSurface for Face {
    fn all_half_edges_with_surface(
        &self,
    ) -> impl Iterator<Item = (Handle<HalfEdge>, Handle<Surface>)> {
        self.region().all_cycles().flat_map(|cycle| {
            cycle
                .half_edges()
                .iter()
                .cloned()
                .map(|half_edge| (half_edge, self.surface().clone()))
        })
    }
}

impl AllHalfEdgesWithSurface for Shell {
    fn all_half_edges_with_surface(
        &self,
    ) -> impl Iterator<Item = (Handle<HalfEdge>, Handle<Surface>)> {
        self.faces()
            .into_iter()
            .flat_map(|face| face.all_half_edges_with_surface())
    }
}
