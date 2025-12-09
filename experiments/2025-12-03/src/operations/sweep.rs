use fj_math::Vector;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
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

    let e12 = vertex_to_half_edge(v1, path, vertices, half_edges);
    let [_, v2] = half_edges[e12].vertices;

    let e23 = {
        let v0_to_v1 = vertices[v1].position - vertices[v0].position;
        vertex_to_half_edge(v2, -v0_to_v1, vertices, half_edges)
    };
    let [_, v3] = half_edges[e23].vertices;

    let e30 = half_edges.push(HalfEdge { vertices: [v3, v0] });

    let t012 = triangles.push([v0, v1, v2], vertices);
    let t023 = triangles.push([v0, v2, v3], vertices);

    faces.push(Face {
        boundary: [e01, e12, e23, e30],
        triangles: [t012, t023],
    })
}
