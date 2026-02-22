use std::collections::BTreeMap;

use crate::{
    math::Point,
    new::topology::{HalfEdge, Handle, Store, Vertex},
};

/// # Connect two primitives, creating a new one
#[derive(Default)]
pub struct Connect {
    vertices: BTreeMap<[Handle<Vertex>; 2], Handle<HalfEdge>>,
}

impl Connect {
    /// # Construct a new instance of `Connect`
    pub fn new() -> Self {
        Self::default()
    }

    /// # Connect two vertices, creating a half-edge
    ///
    /// This function caches its result. If you call it multiple times with the
    /// same two vertices in the same order, it will return a handle to the same
    /// half-edge both times.
    pub fn vertices(
        &mut self,
        vertices: [Handle<Vertex>; 2],
        approx: impl IntoIterator<Item = Point<3>>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Handle<HalfEdge> {
        if let Some(half_edge) = self.vertices.get(&vertices).copied() {
            return half_edge;
        }

        let half_edge = half_edges.push(HalfEdge {
            boundary: vertices,
            approx: approx.into_iter().collect(),
        });
        self.vertices.insert(vertices, half_edge);

        half_edge
    }
}
