use crate::{
    objects::{
        geometry::{Triangle, Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    store::{Index, Store},
};

pub fn triangle(t012: Index<Triangle>, triangles: &Triangles) -> Triangle {
    let [v0, v1, v2] = triangles[t012].vertices;

    Triangle {
        vertices: [v0, v2, v1],
    }
}

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
    let boundary = faces[f0123]
        .boundary
        .iter()
        .copied()
        .map(|e| {
            let half_edge = half_edge(e, half_edges);
            half_edges.push(half_edge)
        })
        .rev()
        .collect();

    let triangles = faces[f0123]
        .triangles
        .iter()
        .copied()
        .map(|t| {
            let triangle = triangle(t, triangles);
            triangles.push(triangle, vertices)
        })
        .rev()
        .collect();

    faces.push(Face {
        boundary,
        triangles,
    })
}
