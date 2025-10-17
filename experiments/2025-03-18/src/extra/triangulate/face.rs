use std::{
    collections::{BTreeSet, VecDeque},
    f64::consts::PI,
    mem,
};

use fj_interop::{Color, MeshTriangle, Tolerance, TriMesh};
use fj_math::{Aabb, Point, Triangle};
use geo::{Contains, Coord, LineString, Polygon};

use crate::{
    approx::{
        delaunay::triangles, face::FaceApproxPoints, half_edge::HalfEdgeApprox,
        point::ApproxPoint, surface::SurfaceApprox,
    },
    topology::face::Face,
};

pub fn triangulate_face(
    face: &Face,
    tolerance: impl Into<Tolerance>,
) -> TriMesh {
    let tolerance = tolerance.into();

    let mut surface_approx = {
        // This happens to be big enough for the current model. But
        // eventually, we need a solution here that works for _any_ model.
        let size = PI;

        let boundary = Aabb {
            min: Point::from([-size, -size]),
            max: Point::from([size, size]),
        };

        SurfaceApprox::new(&face.surface, &boundary, tolerance)
    };

    let half_edges =
        face.half_edges_with_end_vertex()
            .map(|half_edge_with_end_vertex| {
                HalfEdgeApprox::from_half_edge_with_end_vertex(
                    half_edge_with_end_vertex,
                    tolerance,
                )
            });
    let face_approx_points = FaceApproxPoints::from_half_edge_approx(
        half_edges,
        &mut surface_approx,
        tolerance,
    );

    let polygon_from_half_edges =
        polygon_from_half_edges(&face_approx_points.points);

    let surface_points = surface_approx
        .points()
        .filter(|point| {
            polygon_from_half_edges.contains(&Coord {
                x: point.local.u.into_f64(),
                y: point.local.v.into_f64(),
            })
        })
        .copied();

    let triangles = triangles(face_approx_points.points, surface_points)
        .into_iter()
        .filter(|triangle| {
            let points = triangle.map(|point| point.local);
            let triangle = Triangle { points };

            let [x, y] =
                triangle.center().coords.components.map(|s| s.into_f64());

            polygon_from_half_edges.contains(&Coord { x, y })
        })
        .map(|triangle| {
            let points = triangle.map(|point| point.global);

            MeshTriangle {
                inner: Triangle { points },
                is_internal: face.is_internal,
                color: Color::default(),
            }
        });

    let mut mesh = TriMesh::new();
    mesh.triangles.extend(triangles);

    mesh
}

fn polygon_from_half_edges(
    points_from_half_edges: &[ApproxPoint<2>],
) -> Polygon {
    // This is a placeholder implementation that is not well-tested and probably
    // doesn't support polygons with multiple holes.

    let mut line_strings = VecDeque::new();
    let mut current_line_string = Vec::new();
    let mut visited_points = BTreeSet::new();

    for point in points_from_half_edges {
        if visited_points.contains(point) {
            line_strings.push_back(mem::take(&mut current_line_string));
            continue;
        }

        let [x, y] = point.local.coords.components.map(|s| s.into_f64());
        current_line_string.push(Coord { x, y });
        visited_points.insert(point);
    }

    let (exterior, interiors) = if let Some(exterior) = line_strings.pop_front()
    {
        line_strings.push_back(mem::take(&mut current_line_string));

        let exterior = LineString::new(exterior);
        let interiors = line_strings
            .into_iter()
            .filter_map(|line_string| {
                (!line_string.is_empty())
                    .then_some(LineString::new(line_string))
            })
            .collect();

        (exterior, interiors)
    } else {
        let exterior = LineString::new(current_line_string);
        let interiors = Vec::new();

        (exterior, interiors)
    };

    Polygon::new(exterior, interiors)
}
