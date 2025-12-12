use fj_math::Vector;
use itertools::Itertools;

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
    half_edges.push(HalfEdge { boundary: [v0, v1] })
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
        .boundary
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
    bottom: Index<Face>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
    solids: &mut Store<Solid>,
) -> Index<Solid> {
    let path = path.into();

    let bottom_inv =
        reverse::face(bottom, vertices, triangles, half_edges, faces);
    let bottom_edges_for_sides = faces[bottom_inv].boundary;

    let bottom_vertices = bottom_edges_for_sides.map(|e| {
        let [v, _] = half_edges[e].boundary;
        v
    });

    let top_vertices = bottom_vertices.map(|v| {
        let position = vertices[v].position + path;
        vertices.push(Vertex { position })
    });

    let top_edges_for_top = top_vertices
        .into_iter()
        .circular_tuple_windows()
        .map(|(v0, v1)| half_edges.push(HalfEdge { boundary: [v0, v1] }))
        .collect_array()
        .expect("Original array had four entries; output must have the same.");

    let top = face::from_four_half_edges(
        top_edges_for_top,
        vertices,
        half_edges,
        triangles,
        faces,
    );

    let [v0, v1, v2, v3] = bottom_vertices;
    let [v4, v5, v6, v7] = top_vertices;

    let e04 = half_edges.push(HalfEdge { boundary: [v0, v4] });
    let e15 = half_edges.push(HalfEdge { boundary: [v1, v5] });
    let e26 = half_edges.push(HalfEdge { boundary: [v2, v6] });
    let e37 = half_edges.push(HalfEdge { boundary: [v3, v7] });

    let [e45, e56, e67, e74] = top_edges_for_top;

    let e54 = reverse::half_edge(e45, half_edges);
    let e65 = reverse::half_edge(e56, half_edges);
    let e76 = reverse::half_edge(e67, half_edges);
    let e47 = reverse::half_edge(e74, half_edges);

    let e40 = reverse::half_edge(e04, half_edges);
    let e51 = reverse::half_edge(e15, half_edges);
    let e62 = reverse::half_edge(e26, half_edges);
    let e73 = reverse::half_edge(e37, half_edges);

    let [e01, e12, e23, e30] = bottom_edges_for_sides;

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
        boundary: [bottom, f0154, f1265, f2376, f3047, top],
    })
}
