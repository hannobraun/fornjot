use fj_math::Triangle;

use crate::{
    objects::{
        geometry::{Geometry, Triangles},
        topology::{Face, HalfEdge},
    },
    store::{Index, Store},
};

pub fn triangle(
    t012: Index<Triangle<3>>,
    triangles: &Triangles,
) -> Triangle<3> {
    let [p0, p1, p2] = triangles[t012].points;

    Triangle {
        points: [p0, p2, p1],
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
    geometry: &mut Geometry,
    half_edges: &mut Store<HalfEdge>,
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
            let triangle = triangle(t, &geometry.triangles);
            geometry.triangles.push(triangle)
        })
        .rev()
        .collect();

    Face {
        boundary,
        triangles,
    }
}
