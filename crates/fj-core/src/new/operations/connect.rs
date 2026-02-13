use std::collections::BTreeMap;

use fj_math::Point;

use crate::new::topology::{HalfEdge, Handle, Store, Vertex};

pub struct Connect {
    vertices: BTreeMap<[Handle<Vertex>; 2], Handle<HalfEdge>>,
}

impl Connect {
    #[allow(clippy::new_without_default)] // temporary, during transition
    pub fn new() -> Self {
        Self {
            vertices: BTreeMap::new(),
        }
    }

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
