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

    let f1045 = sweep::half_edge_to_face(
        e10, path, vertices, triangles, half_edges, faces,
    );

    let f4037 = {
        let [v3, _] = half_edges[e32].vertices;

        let [_, e04, _, _] = faces[f1045].boundary;
        let e40 = reverse::half_edge(e04, half_edges);

        let v7 = vertices.push(vertices[v3].position + path);

        face::from_two_half_edges_and_vertex(
            [e40, e03],
            v7,
            vertices,
            triangles,
            half_edges,
            faces,
        )
    };

    let f7326 = {
        let [v2, _] = half_edges[e21].vertices;

        let [_, _, e37, _] = faces[f4037].boundary;
        let e73 = reverse::half_edge(e37, half_edges);

        let v6 = vertices.push(vertices[v2].position + path);

        face::from_two_half_edges_and_vertex(
            [e73, e32],
            v6,
            vertices,
            triangles,
            half_edges,
            faces,
        )
    };

    let f6215 = {
        let [_, _, e26, _] = faces[f7326].boundary;
        let e62 = reverse::half_edge(e26, half_edges);

        let [_, _, _, e51] = faces[f1045].boundary;
        let e15 = reverse::half_edge(e51, half_edges);

        face::from_three_half_edges(
            [e62, e21, e15],
            vertices,
            triangles,
            half_edges,
            faces,
        )
    };

    let f1573 = {
        let [_, _, _, e74] = faces[f4037].boundary;
        let e47 = reverse::half_edge(e74, half_edges);

        let [_, _, _, e67] = faces[f7326].boundary;
        let e76 = reverse::half_edge(e67, half_edges);

        let [_, _, _, e56] = faces[f6215].boundary;
        let e65 = reverse::half_edge(e56, half_edges);

        let [_, _, e13, _] = faces[f1045].boundary;
        let e31 = reverse::half_edge(e13, half_edges);

        face::from_four_half_edges(
            [e47, e76, e65, e31],
            vertices,
            half_edges,
            triangles,
            faces,
        )
    };

    solids.push(Solid {
        boundary: [f0123, f1045, f4037, f7326, f6215, f1573],
    })
}
