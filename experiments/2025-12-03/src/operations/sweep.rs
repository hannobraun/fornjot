use fj_math::Vector;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge, Solid},
    },
    operations::{face, reverse},
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

    let [v3, v2] = half_edges[e01]
        .vertices
        .map(|v| vertices.push(vertices[v].position + path));

    face::from_half_edge_and_two_vertices(
        e01,
        [v2, v3],
        vertices,
        triangles,
        half_edges,
        faces,
    )
}

pub fn face_to_solid(
    f0321: Index<Face>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
    solids: &mut Store<Solid>,
) -> Index<Solid> {
    let path = path.into();

    let f0123 = reverse::face(f0321, vertices, triangles, half_edges, faces);
    let [e01, e12, e23, e30] = faces[f0123].boundary;

    let [v0, _] = half_edges[e01].vertices;
    let [v1, _] = half_edges[e12].vertices;
    let [v2, _] = half_edges[e23].vertices;
    let [v3, _] = half_edges[e30].vertices;

    let v4 = vertices.push(vertices[v0].position + path);
    let v5 = vertices.push(vertices[v1].position + path);
    let v6 = vertices.push(vertices[v2].position + path);
    let v7 = vertices.push(vertices[v3].position + path);

    let e45 = half_edges.push(HalfEdge { vertices: [v4, v5] });
    let e56 = half_edges.push(HalfEdge { vertices: [v5, v6] });
    let e67 = half_edges.push(HalfEdge { vertices: [v6, v7] });
    let e74 = half_edges.push(HalfEdge { vertices: [v7, v4] });

    let f4567 = face::from_four_half_edges(
        [e45, e56, e67, e74],
        vertices,
        half_edges,
        triangles,
        faces,
    );

    let e04 = half_edges.push(HalfEdge { vertices: [v0, v4] });
    let e15 = half_edges.push(HalfEdge { vertices: [v1, v5] });
    let e26 = half_edges.push(HalfEdge { vertices: [v2, v6] });
    let e37 = half_edges.push(HalfEdge { vertices: [v3, v7] });

    let e54 = reverse::half_edge(e45, half_edges);
    let e65 = reverse::half_edge(e56, half_edges);
    let e76 = reverse::half_edge(e67, half_edges);
    let e47 = reverse::half_edge(e74, half_edges);

    let e40 = reverse::half_edge(e04, half_edges);
    let e51 = reverse::half_edge(e15, half_edges);
    let e62 = reverse::half_edge(e26, half_edges);
    let e73 = reverse::half_edge(e37, half_edges);

    let f0154 = face::from_four_half_edges(
        [e01, e15, e54, e40],
        vertices,
        half_edges,
        triangles,
        faces,
    );
    let f1265 = face::from_four_half_edges(
        [e12, e26, e65, e51],
        vertices,
        half_edges,
        triangles,
        faces,
    );
    let f2376 = face::from_four_half_edges(
        [e23, e37, e76, e62],
        vertices,
        half_edges,
        triangles,
        faces,
    );
    let f3047 = face::from_four_half_edges(
        [e30, e04, e47, e73],
        vertices,
        half_edges,
        triangles,
        faces,
    );

    solids.push(Solid {
        boundary: [f0321, f0154, f1265, f2376, f3047, f4567],
    })
}
