use fj_math::Vector;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge, Solid},
    },
    operations::{face, reverse, sweep},
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
    f0123: Index<Face>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
    solids: &mut Store<Solid>,
) -> Index<Solid> {
    let path = path.into();

    // Prepare all the bottom edges we're going to need for the side faces.
    let [e03, e32, e21, e10] = {
        let f0321 =
            reverse::face(f0123, vertices, triangles, half_edges, faces);

        faces[f0321].boundary
    };

    // Sweep lower-left edge into left face.
    let f2013 = sweep::half_edge_to_face(
        e10, path, vertices, triangles, half_edges, faces,
    );

    // Complete front face from the parts we already have.
    let f1045 = {
        let [v4, _] = half_edges[e32].vertices;

        let [_, e01, _, _] = faces[f2013].boundary;
        let e10 = reverse::half_edge(e01, half_edges);

        let v5 = vertices.push(vertices[v4].position + path);

        face::from_two_half_edges_and_vertex(
            [e10, e03],
            v5,
            vertices,
            triangles,
            half_edges,
            faces,
        )
    };

    // Complete right face from the parts we already have.
    let f5467 = {
        let [v6, _] = half_edges[e21].vertices;

        let [_, _, e45, _] = faces[f1045].boundary;
        let e54 = reverse::half_edge(e45, half_edges);

        let v7 = vertices.push(vertices[v6].position + path);

        face::from_two_half_edges_and_vertex(
            [e54, e32],
            v7,
            vertices,
            triangles,
            half_edges,
            faces,
        )
    };

    // Complete back face from the parts we already have.
    let f7623 = {
        let [_, _, e67, _] = faces[f5467].boundary;
        let e76 = reverse::half_edge(e67, half_edges);

        let [_, _, _, e32] = faces[f2013].boundary;
        let e23 = reverse::half_edge(e32, half_edges);

        face::from_three_half_edges(
            [e76, e21, e23],
            vertices,
            triangles,
            half_edges,
            faces,
        )
    };

    // Complete top face from the parts we already have.
    let f1573 = {
        let [_, _, _, e51] = faces[f1045].boundary;
        let e15 = reverse::half_edge(e51, half_edges);

        let [_, _, _, e75] = faces[f5467].boundary;
        let e57 = reverse::half_edge(e75, half_edges);

        let [_, _, _, e37] = faces[f7623].boundary;
        let e73 = reverse::half_edge(e37, half_edges);

        let [_, _, e13, _] = faces[f2013].boundary;
        let e31 = reverse::half_edge(e13, half_edges);

        face::from_four_half_edges(
            [e15, e57, e73, e31],
            vertices,
            half_edges,
            triangles,
            faces,
        )
    };

    solids.push(Solid {
        boundary: [f0123, f2013, f1045, f5467, f7623, f1573],
    })
}
