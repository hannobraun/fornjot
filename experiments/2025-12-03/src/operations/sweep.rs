use std::collections::BTreeMap;

use fj_math::Vector;
use itertools::Itertools;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge, Solid},
    },
    operations::{
        reverse,
        sketch::{Sketch, Surface},
    },
    store::{Index, Store},
};

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

    let top_vertices = {
        let mut cache = BTreeMap::new();

        bottom_vertices
            .iter()
            .copied()
            .map(|bottom| {
                if let Some(top) = cache.get(&bottom).copied() {
                    return top;
                }

                let top = vertices.push(Vertex {
                    position: vertices[bottom].position + path,
                });
                cache.insert(bottom, top);

                top
            })
            .collect::<Vec<_>>()
    };

    let top = {
        let boundary = top_vertices
            .iter()
            .copied()
            .circular_tuple_windows()
            .map(|(v0, v1)| half_edges.push(HalfEdge { boundary: [v0, v1] }))
            .collect_array()
            .expect(
                "Original array had four entries; output must have the same.",
            );

        let [e01, e12, e23, e30] = boundary;

        let [v0, v1] = half_edges[e01].boundary;
        let [_, v3] = half_edges[e23].boundary;

        let surface = Surface {
            origin: vertices[v0].position,
            axes: [
                vertices[v1].position - vertices[v0].position,
                vertices[v3].position - vertices[v0].position,
            ],
        };

        Sketch::new()
            .line_to_with_half_edge([1., 0.], e01)
            .line_to_with_half_edge([1., 1.], e12)
            .line_to_with_half_edge([0., 1.], e23)
            .line_to_with_half_edge([0., 0.], e30)
            .into_face(surface, vertices, triangles, half_edges, faces)
    };

    let top_edges_for_sides = faces[top]
        .boundary
        .iter()
        .copied()
        .map(|e| {
            let half_edge = reverse::half_edge(e, half_edges);
            half_edges.push(half_edge)
        })
        .collect::<Vec<_>>();

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
            let left = half_edges.push(left);

            let [v0, v1] = half_edges[bottom].boundary;
            let [_, v3] = half_edges[top].boundary;

            let surface = Surface {
                origin: vertices[v0].position,
                axes: [
                    vertices[v1].position - vertices[v0].position,
                    vertices[v3].position - vertices[v0].position,
                ],
            };

            Sketch::new()
                .line_to_with_half_edge([1., 0.], bottom)
                .line_to_with_half_edge([1., 1.], right)
                .line_to_with_half_edge([0., 1.], top)
                .line_to_with_half_edge([0., 0.], left)
                .into_face(surface, vertices, triangles, half_edges, faces)
        });

    let all_faces = [bottom, top].into_iter().chain(side_faces).collect();

    solids.push(Solid {
        boundary: all_faces,
    })
}
