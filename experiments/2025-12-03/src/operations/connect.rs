use crate::{
    objects::{geometry::Vertex, topology::HalfEdge},
    store::{Index, Store},
};

pub struct Connect {}

impl Connect {
    pub fn vertices_along_line(
        &mut self,
        vertices: [Index<Vertex>; 2],
        half_edges: &mut Store<HalfEdge>,
    ) -> Index<HalfEdge> {
        half_edges.push(HalfEdge { boundary: vertices })
    }
}
