use crate::{
    objects::{geometry::Vertex, topology::HalfEdge},
    store::{Index, Store},
};

pub fn vertices_along_line(
    vertices: [Index<Vertex>; 2],
    half_edges: &mut Store<HalfEdge>,
) -> Index<HalfEdge> {
    half_edges.push(HalfEdge { boundary: vertices })
}
