use crate::{
    geometry::SurfacePath, objects::HalfEdge, operations::insert::Insert,
    storage::Handle, Core,
};

/// Update the geometry of a [`HalfEdge`]
pub trait UpdateHalfEdgeGeometry {
    /// Update the path of the edge
    #[must_use]
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
        core: &mut Core,
    ) -> Self;
}

impl UpdateHalfEdgeGeometry for Handle<HalfEdge> {
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
        core: &mut Core,
    ) -> Self {
        let path = update(self.path());

        HalfEdge::new(
            path,
            self.boundary(),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
        .insert(core)
    }
}
