use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Surface, Vertex},
};

use super::ApproxPoint;

/// # Approximate a vertex position
pub fn approx_vertex(
    vertex: Handle<Vertex>,
    surface: &Handle<Surface>,
    position_surface: Point<2>,
    cache: &mut VertexApproxCache,
    geometry: &Geometry,
) -> ApproxPoint<2> {
    let position_global = match cache.get(&vertex) {
        Some(position) => position,
        None => {
            let position_global = geometry
                .of_surface(surface)
                .point_from_surface_coords(position_surface);
            cache.insert(vertex, position_global)
        }
    };

    ApproxPoint::new(position_surface, position_global)
}

/// Cache for vertex approximations
#[derive(Default)]
pub struct VertexApproxCache {
    inner: BTreeMap<Handle<Vertex>, Point<3>>,
}

impl VertexApproxCache {
    /// Get an approximated vertex from the cache
    pub fn get(&self, handle: &Handle<Vertex>) -> Option<Point<3>> {
        self.inner.get(handle).cloned()
    }

    /// Insert an approximated vertex into the cache
    pub fn insert(
        &mut self,
        handle: Handle<Vertex>,
        position: Point<3>,
    ) -> Point<3> {
        self.inner.insert(handle, position).unwrap_or(position)
    }
}
