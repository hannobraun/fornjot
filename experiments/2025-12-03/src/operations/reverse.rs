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
    face: &Face,
    vertices: &Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    _: &Faces,
) -> Face {
    let boundary = face
        .boundary
        .iter()
        .copied()
        .map(|e| {
            let half_edge = half_edge(e, half_edges);

            if let Some(index) = face
                .boundary
                .iter()
                .copied()
                .find(|&index| half_edges[index] == half_edge)
            {
                index
            } else {
                half_edges.push(half_edge)
            }
        })
        .rev()
        .collect();

    let triangles = face
        .triangles
        .iter()
        .copied()
        .map(|t| {
            let triangle = triangle(t, triangles);
            triangles.push(triangle, vertices)
        })
        .rev()
        .collect();

    Face {
        boundary,
        triangles,
    }
}
