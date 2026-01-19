use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    store::{Index, Store},
    topology::{HalfEdge, Vertex},
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

    pub fn vertices(
        &mut self,
        vertices: [Index<Vertex>; 2],
        approx: impl IntoIterator<Item = Point<3>>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Index<HalfEdge> {
        if let Some(half_edge) =
            self.vertices_along_line.get(&vertices).copied()
        {
            return half_edge;
        }

        let half_edge = half_edges.push(HalfEdge {
            boundary: vertices,
            approx: approx.into_iter().collect(),
        });
        self.vertices_along_line.insert(vertices, half_edge);

        half_edge
    }
}
