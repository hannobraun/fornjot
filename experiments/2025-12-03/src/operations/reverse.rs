use fj_math::Triangle;

use crate::{
    objects::topology::{Face, HalfEdge},
    store::Store,
};

pub fn triangle(triangle: Triangle<3>) -> Triangle<3> {
    let [p0, p1, p2] = triangle.points;

    Triangle {
        points: [p0, p2, p1],
    }
}

pub fn half_edge(half_edge: &HalfEdge, _: &Store<HalfEdge>) -> HalfEdge {
    let [v0, v1] = half_edge.boundary;
    HalfEdge { boundary: [v1, v0] }
}

pub fn face(face: &Face, half_edges: &mut Store<HalfEdge>) -> Face {
    let boundary = face
        .boundary
        .iter()
        .copied()
        .map(|e| {
            let half_edge = half_edge(&half_edges[e], half_edges);

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

    let approx = face.approx.iter().copied().map(triangle).rev().collect();

    Face { boundary, approx }
}
