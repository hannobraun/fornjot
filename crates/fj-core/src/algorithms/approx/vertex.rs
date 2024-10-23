use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::{util::tri_mesh::convert_point_surface_to_global, Geometry},
    storage::Handle,
    topology::{Curve, Surface, Vertex},
};

use super::{ApproxPoint, Tolerance};

/// # Approximate a vertex position
pub fn approx_vertex(
    vertex: Handle<Vertex>,
    curve: &Handle<Curve>,
    surface: &Handle<Surface>,
    position_curve: Point<1>,
    tolerance: impl Into<Tolerance>,
    cache: &mut VertexApproxCache,
    geometry: &Geometry,
) -> ApproxPoint<1> {
    let position_surface = geometry
        .of_curve(curve)
        .unwrap()
        .local_on(surface)
        .unwrap()
        .path
        .point_from_path_coords(position_curve);

    let position_global = match cache.get(&vertex) {
        Some(position) => position,
        None => {
            let position_global = convert_point_surface_to_global(
                &geometry.generator_for_surface(surface).unwrap().generator,
                position_surface,
                tolerance,
                geometry,
            );
            cache.insert(vertex, position_global)
        }
    };

    ApproxPoint::new(position_curve, position_global)
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
