use crate::{
    storage::Handle,
    topology::{Face, HalfEdge, Shell, Surface},
};

/// Access all half-edges referenced by an object, and the surface they're on
pub trait AllHalfEdgesWithSurface {
    /// Access all half-edges of the object, and the surface they're on
    fn all_half_edges_with_surface(
        &self,
        result: &mut Vec<(Handle<HalfEdge>, Handle<Surface>)>,
    );
}

impl AllHalfEdgesWithSurface for Face {
    fn all_half_edges_with_surface(
        &self,
        result: &mut Vec<(Handle<HalfEdge>, Handle<Surface>)>,
    ) {
        self.region()
            .all_cycles()
            .flat_map(|cycle| {
                cycle
                    .half_edges()
                    .iter()
                    .cloned()
                    .map(|half_edge| (half_edge, self.surface().clone()))
            })
            .for_each(|r| result.push(r))
    }
}

impl AllHalfEdgesWithSurface for Shell {
    fn all_half_edges_with_surface(
        &self,
        result: &mut Vec<(Handle<HalfEdge>, Handle<Surface>)>,
    ) {
        for face in self.faces() {
            face.all_half_edges_with_surface(result);
        }
    }
}
