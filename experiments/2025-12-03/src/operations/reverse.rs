use itertools::Itertools;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    operations::sketch::Sketch,
    store::{Index, Store},
};

pub fn half_edge(
    e01: Index<HalfEdge>,
    half_edges: &Store<HalfEdge>,
) -> HalfEdge {
    let [v0, v1] = half_edges[e01].boundary;
    HalfEdge { boundary: [v1, v0] }
}

pub fn face(
    f0123: Index<Face>,
    vertices: &Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
) -> Index<Face> {
    let [e10, e21, e32, e03] = faces[f0123]
        .boundary
        .iter()
        .copied()
        .map(|e| {
            let half_edge = half_edge(e, half_edges);
            half_edges.push(half_edge)
        })
        .collect_array::<4>()
        .unwrap();

    Sketch::new()
        .push_half_edge(e03)
        .push_half_edge(e32)
        .push_half_edge(e21)
        .push_half_edge(e10)
        .build(vertices, half_edges, triangles, faces)
}
