use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    operations::face,
    store::{Index, Store},
};

pub fn half_edge(e: Index<HalfEdge>, half_edges: &Store<HalfEdge>) -> HalfEdge {
    let [v0, v1] = half_edges[e].boundary;
    HalfEdge { boundary: [v1, v0] }
}

pub fn face(
    f0123: Index<Face>,
    vertices: &Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
) -> Index<Face> {
    let [e10, e21, e32, e03] = faces[f0123].boundary.map(|e| {
        let half_edge = half_edge(e, half_edges);
        half_edges.push(half_edge)
    });

    face::from_four_half_edges(
        [e03, e32, e21, e10],
        vertices,
        half_edges,
        triangles,
        faces,
    )
}
