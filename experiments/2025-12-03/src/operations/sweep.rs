use fj_math::{Scalar, Vector};

use crate::{
    objects::topology::{Face, HalfEdge, Solid, Vertex},
    operations::{
        connect::Connect,
        reverse,
        sketch::{Sketch, Surface},
        translate,
    },
    store::{Index, Store},
};

pub fn face_to_solid(
    bottom: Index<Face>,
    path: impl Into<Vector<3>>,
    tolerance: impl Into<Scalar>,
    vertices: &mut Store<Vertex>,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Store<Face>,
    solids: &mut Store<Solid>,
) -> Index<Solid> {
    let path = path.into();
    let tolerance = tolerance.into();

    let mut connect = Connect::new();

    let bottom_inv = reverse::face(&faces[bottom], half_edges);

    let top = {
        let top = translate::face(&bottom_inv, path, vertices, half_edges);
        faces.push(top)
    };

    let bottom_edges_for_sides = bottom_inv.boundary.clone();
    let top_edges_for_sides = {
        let mut top_edges =
            reverse::face(&faces[top], half_edges).boundary.clone();

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
                connect.vertices_along_line([v_bottom, v_top], half_edges)
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
            connect.vertices_along_line([v_top, v_bottom], half_edges)
        })
        .collect::<Vec<_>>();

    let side_faces = bottom_edges_for_sides
        .into_iter()
        .zip(side_edges_going_up)
        .zip(top_edges_for_sides)
        .zip(side_edges_going_down)
        .map(|(((bottom, right), top), left)| {
            let [[p0, p1], [_, p3]] = [bottom, top].map(|half_edge| {
                half_edges[half_edge]
                    .boundary
                    .map(|vertex| vertices[vertex].point)
            });

            let surface = Surface {
                origin: p0,
                axes: [p1 - p0, p3 - p0],
            };

            Sketch::start_at([0., 0.])
                .line_to_with_half_edge([1., 0.], bottom)
                .line_to_with_half_edge([1., 1.], right)
                .line_to_with_half_edge([0., 1.], top)
                .line_to_with_half_edge([0., 0.], left)
                .into_face(surface, tolerance, vertices, half_edges, faces)
        });

    let all_faces = [bottom, top].into_iter().chain(side_faces).collect();

    solids.push(Solid {
        boundary: all_faces,
    })
}
