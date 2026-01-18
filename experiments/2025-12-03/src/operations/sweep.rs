use fj_math::Point;

use crate::{
    geometry::curve::LineSegment,
    helpers::approx_face,
    operations::{connect::Connect, reverse, translate},
    store::{Index, Store},
    topology::{Face, HalfEdge, Solid, Vertex},
};

pub fn face_to_solid(
    bottom: Index<Face>,
    curve: &LineSegment,
    vertices: &mut Store<Vertex>,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Store<Face>,
    solids: &mut Store<Solid>,
) -> Index<Solid> {
    let path = curve.end;

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
            let approx = approx_face(
                [0., 0.],
                vec![
                    (
                        Point::from([1., 0.]),
                        bottom,
                        local_approx_coords(bottom, 0., half_edges),
                    ),
                    (Point::from([1., 1.]), right, Vec::new()),
                    (Point::from([0., 1.]), top, {
                        let mut approx =
                            local_approx_coords(top, 1., half_edges);
                        approx.reverse();
                        approx
                    }),
                    (Point::from([0., 0.]), left, Vec::new()),
                ],
                vertices,
                half_edges,
            );

            faces.push(Face {
                boundary: vec![bottom, right, top, left],
                approx,
            })
        });

    let all_faces = [bottom, top].into_iter().chain(side_faces).collect();

    solids.push(Solid {
        boundary: all_faces,
    })
}

fn local_approx_coords(
    half_edge: Index<HalfEdge>,
    v: f64,
    half_edges: &Store<HalfEdge>,
) -> Vec<Point<2>> {
    let len = half_edges[half_edge].approx.len();
    let increment = 1. / (len as f64 + 1.);

    (1..=len)
        .map(|i| {
            let u = increment * i as f64;
            Point::from([u, v])
        })
        .collect()
}
