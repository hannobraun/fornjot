use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    operations::face,
    store::{Index, Store},
};

pub fn half_edge(
    e: Index<HalfEdge>,
    half_edges: &mut Store<HalfEdge>,
) -> Index<HalfEdge> {
    let [v0, v1] = half_edges[e].vertices;
    half_edges.push(HalfEdge { vertices: [v1, v0] })
}

pub fn reverse_face(
    f0123: Index<Face>,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
    triangles: &mut Triangles,
    vertices: &Store<Vertex>,
) -> Index<Face> {
    let [e10, e21, e32, e03] =
        faces[f0123].boundary.map(|e| half_edge(e, half_edges));

    face::from_four_half_edges(
        [e03, e32, e21, e10],
        vertices,
        half_edges,
        triangles,
        faces,
    )
}
