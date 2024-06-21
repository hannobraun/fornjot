use std::collections::BTreeMap;

use fj_math::Point;

use crate::{storage::Handle, topology::Vertex};

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
