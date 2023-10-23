//! Vertex approximation

use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    objects::Vertex,
    storage::{Handle, HandleWrapper},
};

/// Cache for vertex approximations
#[derive(Default)]
pub struct VertexApproxCache {
    inner: BTreeMap<HandleWrapper<Vertex>, Point<3>>,
}

impl VertexApproxCache {
    /// Get an approximated vertex from the cache
    pub fn get(&self, handle: &Handle<Vertex>) -> Option<Point<3>> {
        self.inner.get(&handle.clone().into()).cloned()
    }

    /// Insert an approximated vertex into the cache
    pub fn insert(
        &mut self,
        handle: Handle<Vertex>,
        position: Point<3>,
    ) -> Point<3> {
        self.inner
            .insert(handle.clone().into(), position)
            .unwrap_or(position)
    }
}
