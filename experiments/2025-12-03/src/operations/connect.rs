use std::collections::BTreeMap;

use crate::{
    objects::topology::{HalfEdge, Vertex},
    store::{Index, Store},
};

pub struct Connect {
    vertices_along_line: BTreeMap<[Index<Vertex>; 2], Index<HalfEdge>>,
}

impl Connect {
    pub fn new() -> Self {
        Self {
            vertices_along_line: BTreeMap::new(),
        }
    }

    pub fn vertices_along_line(
        &mut self,
        vertices: [Index<Vertex>; 2],
        half_edges: &mut Store<HalfEdge>,
    ) -> Index<HalfEdge> {
        if let Some(half_edge) =
            self.vertices_along_line.get(&vertices).copied()
        {
            return half_edge;
        }

        let half_edge = half_edges.push(HalfEdge {
            boundary: vertices,
            approx: Vec::new(),
        });
        self.vertices_along_line.insert(vertices, half_edge);

        half_edge
    }
}
