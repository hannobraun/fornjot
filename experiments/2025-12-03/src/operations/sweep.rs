use fj_math::Vector;
use itertools::Itertools;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge, Solid},
    },
    operations::{reverse, sketch::Sketch},
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

    Sketch::start_at([0., 0.])
        .push_half_edge([1., 0.], e01)
        .push_vertex([1., 1.], v2, half_edges)
        .push_vertex([0., 1.], v3, half_edges)
        .close(half_edges)
        .build(vertices, half_edges, triangles, faces)
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
    let bottom_edges_for_sides = faces[bottom_inv].boundary.clone();

    let bottom_vertices = bottom_edges_for_sides
        .iter()
        .copied()
        .map(|e| {
            let [v, _] = half_edges[e].boundary;
            v
        })
        .collect::<Vec<_>>();

    let top_vertices = bottom_vertices
        .iter()
        .copied()
        .map(|v| {
            let position = vertices[v].position + path;
            vertices.push(Vertex { position })
        })
        .collect::<Vec<_>>();

    let top_edges_for_top = top_vertices
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(v0, v1)| half_edges.push(HalfEdge { boundary: [v0, v1] }))
        .collect_array()
        .expect("Original array had four entries; output must have the same.");

    let top = {
        let [e01, e12, e23, e30] = top_edges_for_top;
        Sketch::start_at([0., 0.])
            .push_half_edge([1., 0.], e01)
            .push_half_edge([1., 1.], e12)
            .push_half_edge([0., 1.], e23)
            .push_half_edge([0., 0.], e30)
            .build(vertices, half_edges, triangles, faces)
    };

    let side_edges_going_up = bottom_vertices
        .iter()
        .copied()
        .zip(top_vertices.iter().copied())
        .map(|(v_bottom, v_top)| HalfEdge {
            boundary: [v_bottom, v_top],
        })
        .cycle()
        .skip(1)
        .take(bottom_vertices.len())
        .collect::<Vec<_>>();

    let top_edges_for_sides =
        top_edges_for_top.map(|e| reverse::half_edge(e, half_edges));

    let side_edges_going_down = top_vertices
        .into_iter()
        .zip(bottom_vertices)
        .map(|(v_top, v_bottom)| HalfEdge {
            boundary: [v_top, v_bottom],
        });

    let side_faces = bottom_edges_for_sides
        .into_iter()
        .zip(side_edges_going_up)
        .zip(top_edges_for_sides)
        .zip(side_edges_going_down)
        .map(|(((bottom, right), top), left)| {
            let right = half_edges.push(right);
            let top = half_edges.push(top);
            let left = half_edges.push(left);

            Sketch::start_at([0., 0.])
                .push_half_edge([1., 0.], bottom)
                .push_half_edge([1., 1.], right)
                .push_half_edge([0., 1.], top)
                .push_half_edge([0., 0.], left)
                .build(vertices, half_edges, triangles, faces)
        });

    let all_faces = [bottom, top].into_iter().chain(side_faces).collect();

    solids.push(Solid {
        boundary: all_faces,
    })
}
