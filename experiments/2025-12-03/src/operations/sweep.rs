use fj_math::Point;

use crate::{
    approx::{ApproxPoint, HalfEdgeApprox},
    geometry::curve::Curve,
    helpers::approx_face,
    operations::{connect::Connect, reverse, translate},
    store::{Index, Store},
    topology::{Face, HalfEdge, Solid, Vertex},
};

pub fn face_to_solid(
    bottom: Index<Face>,
    curve: &impl Curve,
    vertices: &mut Store<Vertex>,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Store<Face>,
    solids: &mut Store<Solid>,
) -> Index<Solid> {
    let mut connect = Connect::new();

    let bottom_inv = reverse::face(&faces[bottom], half_edges);

    let top = {
        let top =
            translate::face(&bottom_inv, curve.end(), vertices, half_edges);
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

    let (side_edges_going_up, side_edges_going_down) = {
        let approx = curve.approx();

        let mut side_edges_going_up = bottom_vertices
            .iter()
            .copied()
            .zip(top_vertices.iter().copied())
            .map(|(v_bottom, v_top)| {
                connect.vertices(
                    [v_bottom, v_top],
                    approx
                        .iter()
                        .copied()
                        .map(|vector| vertices[v_bottom].point + vector),
                    half_edges,
                )
            })
            .collect::<Vec<_>>();

        // Both lists of side edges need to line up, so that the same index
        // refers to an edge for the same face. This makes some shuffling
        // necessary.
        side_edges_going_up.rotate_left(1);

        let side_edges_going_down = top_vertices
            .into_iter()
            .zip(bottom_vertices)
            .map(|(v_top, v_bottom)| {
                connect.vertices(
                    [v_top, v_bottom],
                    approx
                        .iter()
                        .copied()
                        .rev()
                        .map(|vector| vertices[v_bottom].point + vector),
                    half_edges,
                )
            })
            .collect::<Vec<_>>();

        (side_edges_going_up, side_edges_going_down)
    };

    let side_faces = bottom_edges_for_sides
        .into_iter()
        .zip(side_edges_going_up)
        .zip(top_edges_for_sides)
        .zip(side_edges_going_down)
        .map(|(((bottom, right), top), left)| {
            let approx = approx_face(vec![
                HalfEdgeApprox {
                    start: ApproxPoint {
                        local: Point::from([0., 0.]),
                        global: vertices[half_edges[bottom].boundary[0]].point,
                    },
                    other: local_approx_coords(
                        bottom,
                        FixedCoord::V { value: 0. },
                        half_edges,
                        false,
                    ),
                },
                HalfEdgeApprox {
                    start: ApproxPoint {
                        local: Point::from([1., 0.]),
                        global: vertices[half_edges[right].boundary[0]].point,
                    },
                    other: local_approx_coords(
                        right,
                        FixedCoord::U { value: 1. },
                        half_edges,
                        false,
                    ),
                },
                HalfEdgeApprox {
                    start: ApproxPoint {
                        local: Point::from([1., 1.]),
                        global: vertices[half_edges[top].boundary[0]].point,
                    },
                    other: local_approx_coords(
                        top,
                        FixedCoord::V { value: 1. },
                        half_edges,
                        true,
                    ),
                },
                HalfEdgeApprox {
                    start: ApproxPoint {
                        local: Point::from([0., 1.]),
                        global: vertices[half_edges[left].boundary[0]].point,
                    },
                    other: local_approx_coords(
                        left,
                        FixedCoord::U { value: 0. },
                        half_edges,
                        true,
                    ),
                },
            ]);

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

pub fn local_approx_coords(
    half_edge: Index<HalfEdge>,
    fixed: FixedCoord,
    half_edges: &Store<HalfEdge>,
    reverse: bool,
) -> Vec<ApproxPoint<2>> {
    let half_edge = &half_edges[half_edge];

    let local = {
        let increment = 1. / (half_edge.approx.len() as f64 + 1.);

        let mut points = (0..half_edge.approx.len())
            .map(|i| increment * (i + 1) as f64)
            .collect::<Vec<_>>();

        if reverse {
            points.reverse();
        }

        points
    };
    let global = half_edge.approx.iter().copied();

    local
        .into_iter()
        .zip(global)
        .map(|(local, global)| {
            let (u, v) = match fixed {
                FixedCoord::U { value } => (value, local),
                FixedCoord::V { value } => (local, value),
            };

            ApproxPoint {
                local: Point::from([u, v]),
                global,
            }
        })
        .collect()
}

pub enum FixedCoord {
    U { value: f64 },
    V { value: f64 },
}
