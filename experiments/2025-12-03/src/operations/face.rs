use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    store::{Index, Store},
};

pub fn from_half_edge_and_two_vertices(
    e01: Index<HalfEdge>,
    [v2, v3]: [Index<Vertex>; 2],
    vertices: &Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
) -> Index<Face> {
    let [v0, v1] = half_edges[e01].boundary;

    let e12 = half_edges.push(HalfEdge { boundary: [v1, v2] });
    let e23 = half_edges.push(HalfEdge { boundary: [v2, v3] });
    let e30 = half_edges.push(HalfEdge { boundary: [v3, v0] });

    from_four_half_edges(
        [e01, e12, e23, e30],
        vertices,
        half_edges,
        triangles,
        faces,
    )
}

pub fn from_four_half_edges(
    [e01, e12, e23, e30]: [Index<HalfEdge>; 4],
    vertices: &Store<Vertex>,
    half_edges: &Store<HalfEdge>,
    triangles: &mut Triangles,
    faces: &mut Faces,
) -> Index<Face> {
    let [v0, v1b] = half_edges[e01].boundary;
    let [v1, v2b] = half_edges[e12].boundary;
    let [v2, v3b] = half_edges[e23].boundary;
    let [v3, v0b] = half_edges[e30].boundary;

    assert_eq!(v0, v0b);
    assert_eq!(v1, v1b);
    assert_eq!(v2, v2b);
    assert_eq!(v3, v3b);

    let t012 = triangles.push([v0, v1, v2], vertices);
    let t123 = triangles.push([v0, v2, v3], vertices);

    faces.push(Face {
        boundary: vec![e01, e12, e23, e30],
        triangles: [t012, t123],
    })
}
