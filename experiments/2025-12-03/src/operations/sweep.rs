use fj_core::new::{
    approx::{ApproxPoint, Axis, HalfEdgeApprox},
    topology::{Face, HalfEdge, Handle, Solid, Store, Vertex},
};
use fj_math::Point;
use itertools::Itertools;

use crate::{
    geometry::curve::Curve,
    helpers::approx_face,
    operations::{connect::Connect, reverse, translate},
};

pub fn face_to_solid(
    bottom: Handle<Face>,
    curve: &impl Curve,
    vertices: &mut Store<Vertex>,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Store<Face>,
    solids: &mut Store<Solid>,
) -> Handle<Solid> {
    let approx = curve.approx();
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
            let boundary = [
                HalfEdgeApprox::from_start_and_axes(
                    [0., 0.],
                    Axis::Uniform { reverse: false },
                    Axis::fixed(0.),
                    bottom,
                    vertices,
                    half_edges,
                ),
                HalfEdgeApprox::from_start_and_axes(
                    [1., 0.],
                    Axis::fixed(1.),
                    Axis::Uniform { reverse: false },
                    right,
                    vertices,
                    half_edges,
                ),
                HalfEdgeApprox::from_start_and_axes(
                    [1., 1.],
                    Axis::Uniform { reverse: true },
                    Axis::fixed(1.),
                    top,
                    vertices,
                    half_edges,
                ),
                HalfEdgeApprox::from_start_and_axes(
                    [0., 1.],
                    Axis::fixed(0.),
                    Axis::Uniform { reverse: true },
                    left,
                    vertices,
                    half_edges,
                ),
            ];
            let surface = {
                let [bottom, right, _, _] = &boundary;

                let u = bottom.inner.iter().map(|point| point.local.u);
                let v = right.inner.iter().map(|point| point.local.v);

                let local =
                    u.cartesian_product(v).map(|(u, v)| Point::from([u, v]));
                let global = bottom
                    .inner
                    .iter()
                    .map(|point| point.global)
                    .cartesian_product(approx.iter().copied())
                    .map(|(point, vector)| point + vector);

                local
                    .zip(global)
                    .map(|(local, global)| ApproxPoint { local, global })
            };

            let approx = approx_face(&boundary, surface);

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
