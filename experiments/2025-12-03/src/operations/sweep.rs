use fj_math::Vector;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    operations::face,
    store::{Index, Store},
};

pub fn vertex_to_half_edge(
    v0: Index<Vertex>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    half_edges: &mut Store<HalfEdge>,
) -> Index<HalfEdge> {
    let path = path.into();

    let v1 = vertices.push(vertices[v0].position + path);
    half_edges.push(HalfEdge { vertices: [v0, v1] })
}

pub fn half_edge_to_face(
    e01: Index<HalfEdge>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
) -> Index<Face> {
    let path = path.into();

    let [v0, v1] = half_edges[e01].vertices;
    let [v3, v2] = [v0, v1].map(|v| vertices.push(vertices[v].position + path));

    face::from_half_edge_and_two_vertices(
        e01,
        [v2, v3],
        vertices,
        triangles,
        half_edges,
        faces,
    )
}
