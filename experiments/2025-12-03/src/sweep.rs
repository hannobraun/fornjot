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
    e0: Index<HalfEdge>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
) -> Index<Face> {
    let path = path.into();

    let [v0, v1] = half_edges[e0].vertices;

    let e1 = vertex_to_half_edge(v1, path, vertices, half_edges);
    let [_, v2] = half_edges[e1].vertices;

    let e2 = {
        let v0_to_v1 = vertices[v1].position - vertices[v0].position;
        vertex_to_half_edge(v2, -v0_to_v1, vertices, half_edges)
    };
    let [_, v3] = half_edges[e2].vertices;

    let e3 = half_edges.push(HalfEdge { vertices: [v3, v0] });

    let t0 = triangles.push([v0, v1, v2], vertices);
    let t1 = triangles.push([v0, v2, v3], vertices);

    faces.push(Face {
        boundary: [e0, e1, e2, e3],
        triangles: [t0, t1],
    })
}
