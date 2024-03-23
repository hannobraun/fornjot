use crate::{
    geometry::{Geometry, HalfEdgeGeometry},
    layers::Layer,
    objects::HalfEdge,
    storage::Handle,
};

/// Update the geometry of a [`HalfEdge`]
pub trait UpdateHalfEdgeGeometry {
    /// Set the path of the half-edge
    fn set_geometry(
        self,
        geometry: HalfEdgeGeometry,
        layer: &mut Layer<Geometry>,
    ) -> Self;
}

impl UpdateHalfEdgeGeometry for Handle<HalfEdge> {
    fn set_geometry(
        self,
        geometry: HalfEdgeGeometry,
        layer: &mut Layer<Geometry>,
    ) -> Self {
        layer.define_half_edge(self.clone(), geometry);
        self
    }
}
