use std::collections::BTreeSet;

use itertools::Itertools;

use crate::core::{
    math::{Point, Scalar},
    new::{
        geometry::Plane,
        operations::Sketch2,
        topology::{Face, HalfFace, Orientation, Topology},
    },
};

pub fn empty() {
    let mut topology = Topology::new();

    let half_face = Sketch2::new().into_half_face(Plane::xy(), &mut topology);

    assert_eq!(half_face.boundary, Vec::new());
    assert_eq!(topology.faces[half_face.face], Face { approx: Vec::new() });
    assert_eq!(half_face.orientation, Orientation::Nominal);
}

pub fn triangle() {
    let mut topology = Topology::new();

    let half_face = Sketch2::new()
        .line_to([1., 0.])
        .line_to([0., 1.])
        .line_to([0., 0.])
        .into_half_face(Plane::xy(), &mut topology);

    assert_eq!(half_face.boundary.len(), 3);
    check_connecting_vertices(&half_face, &topology);
    check_approx(
        &half_face,
        &topology,
        1,
        [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
    );
    assert_eq!(half_face.orientation, Orientation::Nominal);
}

pub fn circle() {
    let mut topology = Topology::new();

    let radius = 1.;
    let tolerance = 0.35; // chosen to approximate circle in two triangles

    let half_face = Sketch2::new()
        .arc_to(radius, tolerance, [2., 0.])
        .arc_to(radius, tolerance, [0., 0.])
        .into_half_face(Plane::xy(), &mut topology);

    assert_eq!(half_face.boundary.len(), 2);
    check_connecting_vertices(&half_face, &topology);
    check_approx(
        &half_face,
        &topology,
        2,
        [[0., 0., 0.], [1., -1., 0.], [2., 0., 0.], [1., 1., 0.]],
    );
    assert_eq!(half_face.orientation, Orientation::Nominal);
}

fn check_connecting_vertices(half_face: &HalfFace, topology: &Topology) {
    for (prev, half_edge, next) in half_face
        .boundary
        .iter()
        .map(|&half_edge| &topology.half_edges[half_edge])
        .circular_tuple_windows()
    {
        assert_eq!(
            prev.boundary(&topology.edges)[1],
            half_edge.boundary(&topology.edges)[0]
        );
        assert_eq!(
            half_edge.boundary(&topology.edges)[1],
            next.boundary(&topology.edges)[0]
        );

        assert_eq!(half_edge.orientation, Orientation::Nominal);
    }
}

fn check_approx(
    half_face: &HalfFace,
    topology: &Topology,
    num_expected_triangles: usize,
    expected_triangle_points: impl IntoIterator<Item = impl Into<Point<3>>>,
) {
    let face = &topology.faces[half_face.face];
    assert_eq!(face.approx.len(), num_expected_triangles);

    let mut triangle_points = face
        .approx
        .iter()
        .flat_map(|triangle| triangle.points)
        .collect::<BTreeSet<_>>();

    for expected in expected_triangle_points {
        let expected = expected.into();

        let Some(&point) = triangle_points.iter().find(|&&point| {
            (point - expected).magnitude() < Scalar::from(0.001)
        }) else {
            panic!("Could not find expected point `{expected:?}`.");
        };

        assert!(triangle_points.remove(&point));
    }

    assert!(triangle_points.is_empty());
}
