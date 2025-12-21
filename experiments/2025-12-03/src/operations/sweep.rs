use fj_math::Vector;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge, Solid},
    },
    operations::{
        reverse,
        sketch::{Sketch, Surface},
        translate,
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
        reverse::face(&faces[bottom], vertices, triangles, half_edges);

    let top = {
        let top =
            translate::face(&bottom_inv, path, vertices, triangles, half_edges);
        faces.push(top)
    };

    let bottom_edges_for_sides = bottom_inv.boundary.clone();
    let top_edges_for_sides = {
        let mut top_edges =
            reverse::face(&faces[top], vertices, triangles, half_edges)
                .boundary
                .clone();

        top_edges.reverse();

        top_edges
    };

    let bottom_vertices = bottom_edges_for_sides
        .iter()
        .copied()
        .map(|e| {
            let [v, _] = half_edges[e].boundary;
            v
        })
        .collect::<Vec<_>>();
    let top_vertices = top_edges_for_sides
        .iter()
        .copied()
        .map(|e| {
            let [_, v] = half_edges[e].boundary;
            v
        })
        .collect::<Vec<_>>();

    let side_edges_going_up = {
        let mut side_edges_going_up = bottom_vertices
            .iter()
            .copied()
            .zip(top_vertices.iter().copied())
            .map(|(v_bottom, v_top)| {
                half_edges.push(HalfEdge {
                    boundary: [v_bottom, v_top],
                })
            })
            .collect::<Vec<_>>();

        // Both lists of side edges need to line up, so that the same index
        // refers to an edge for the same face. This makes some shuffling
        // necessary.
        side_edges_going_up.rotate_left(1);

        side_edges_going_up
    };

    let side_edges_going_down = top_vertices
        .into_iter()
        .zip(bottom_vertices)
        .map(|(v_top, v_bottom)| {
            half_edges.push(HalfEdge {
                boundary: [v_top, v_bottom],
            })
        })
        .collect::<Vec<_>>();

    let side_faces = bottom_edges_for_sides
        .into_iter()
        .zip(side_edges_going_up)
        .zip(top_edges_for_sides)
        .zip(side_edges_going_down)
        .map(|(((bottom, right), top), left)| {
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
