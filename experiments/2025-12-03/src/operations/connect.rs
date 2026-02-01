use std::collections::BTreeMap;

use fj_math::Point;

use crate::topology::{HalfEdge, Index, Store, Vertex};

pub struct Connect {
    vertices: BTreeMap<[Index<Vertex>; 2], Index<HalfEdge>>,
}

impl Connect {
    pub fn new() -> Self {
        Self {
            vertices: BTreeMap::new(),
        }
    }

    pub fn vertices(
        &mut self,
        vertices: [Index<Vertex>; 2],
        approx: impl IntoIterator<Item = Point<3>>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Index<HalfEdge> {
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
