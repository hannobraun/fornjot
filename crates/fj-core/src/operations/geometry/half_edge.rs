use crate::{
    geometry::{Geometry, HalfEdgeGeometry, SurfacePath},
    layers::Layer,
    objects::HalfEdge,
    storage::Handle,
};

/// Update the geometry of a [`HalfEdge`]
pub trait UpdateHalfEdgeGeometry {
    /// Set the path of the half-edge
    fn set_path(
        self,
        path: SurfacePath,
        geometry: &mut Layer<Geometry>,
    ) -> Self;
}

impl UpdateHalfEdgeGeometry for Handle<HalfEdge> {
    fn set_path(
        self,
        path: SurfacePath,
        geometry: &mut Layer<Geometry>,
    ) -> Self {
        geometry.define_half_edge(self.clone(), HalfEdgeGeometry { path });
        self
    }
}
