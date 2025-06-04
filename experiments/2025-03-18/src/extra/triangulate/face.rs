use std::{
    collections::{BTreeSet, VecDeque},
    mem,
};

use fj_interop::{Color, MeshTriangle, Tolerance, TriMesh};
use fj_math::{Aabb, Point, Triangle};
use geo::{Contains, Coord, LineString, Polygon};
use spade::Triangulation;

use crate::{
    extra::triangulate::triangulate_surface,
    topology::{
        face::{Face, HalfEdgeWithEndVertex},
        surface::Surface,
    },
};

use super::TriangulationPoint;

pub fn triangulate_face(
    face: &Face,
    tolerance: impl Into<Tolerance>,
) -> TriMesh {
    let tolerance = tolerance.into();

    let surface = {
        // This happens to be big enough for the current model. But
        // eventually, we need a solution here that works for _any_ model.
        let size = 4.;

        let boundary = Aabb {
            min: Point::from([-size, -size]),
            max: Point::from([size, size]),
        };

        triangulate_surface(&face.surface, &boundary, tolerance)
    };
    dbg!(surface);

    let mut points_from_half_edges = Vec::new();
    half_edges_to_points(face, &mut points_from_half_edges, tolerance);

    let polygon_from_half_edges =
        polygon_from_half_edges(&points_from_half_edges);

    let mut all_points = points_from_half_edges;
    points_from_surface(
        &face.surface,
        &polygon_from_half_edges,
        &mut all_points,
    );

    let triangles = triangles(&all_points)
        .into_iter()
        .filter(|triangle| {
            let points = triangle.map(|point| point.point_surface);
            let triangle = Triangle { points };

            let [x, y] =
                triangle.center().coords.components.map(|s| s.into_f64());

            polygon_from_half_edges.contains(&Coord { x, y })
        })
        .map(|triangle| {
            let points = triangle.map(|point| point.point_global);

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

fn half_edges_to_points(
    face: &Face,
    target: &mut Vec<TriangulationPoint>,
    tolerance: impl Into<Tolerance>,
) {
    let tolerance = tolerance.into();

    target.extend(
        face.half_edges_with_end_vertex()
            .flat_map(|half_edge_with_end_vertex| {
                approximate_half_edge(half_edge_with_end_vertex, tolerance)
            })
            .map(|point_global| {
                // Here, we project a 3D point (from the vertex) into the face's
                // surface, creating a 2D point. Through the surface, this 2D
                // point has a position in 3D space.
                //
                // But this position isn't necessarily going to be the same as
                // the position of the original 3D point, due to numerical
                // inaccuracy.
                //
                // This doesn't matter. Neither does the fact, that other faces
                // might share the same vertices and project them into their own
                // surfaces, creating more redundancy.
                //
                // The reason that it doesn't, is that we're using the projected
                // 2D points _only_ for this local triangulation. Once that
                // tells us how the different 3D points must connect, we use the
                // original 3D points to build those triangles. We never convert
                // the 2D points back into 3D.
                let point_surface =
                    face.surface.geometry.project_point(point_global);

                TriangulationPoint {
                    point_surface,
                    point_global,
                }
            }),
    )
}

/// # Approximate an half-edge
///
/// The approximation of an half-edge is the approximation of its curve within
/// the boundary defined by the half-edge's start and end vertices, plus the
/// position of the start vertex.
///
/// By including the start vertex and not the end vertex, a whole chain of
/// half-edges can be approximated by simply appending the approximations of
/// each half-edge, without the necessity of any deduplication of points.
fn approximate_half_edge(
    HalfEdgeWithEndVertex {
        half_edge,
        end_vertex,
    }: HalfEdgeWithEndVertex,
    tolerance: Tolerance,
) -> Vec<Point<3>> {
    let [start, end] =
        [&half_edge.start, end_vertex].map(|vertex| vertex.point);

    let boundary_local = [start, end].map(|point_global| {
        half_edge.curve.geometry.project_point(point_global)
    });
    let points_local = half_edge
        .curve
        .geometry
        .approximate(boundary_local, tolerance);

    let mut points_global = vec![start];
    points_global.extend(points_local.into_iter().map(|point_local| {
        half_edge.curve.geometry.point_from_local(point_local)
    }));

    points_global
}

fn polygon_from_half_edges(
    points_from_half_edges: &[TriangulationPoint],
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

        let [x, y] =
            point.point_surface.coords.components.map(|s| s.into_f64());
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

fn points_from_surface(
    surface: &Surface,
    boundary: &Polygon,
    target: &mut Vec<TriangulationPoint>,
) {
    let aabb = Aabb::<2>::from_points(
        boundary
            .exterior()
            .points()
            .map(|point| Point::from([point.x(), point.y()])),
    );

    target.extend(surface.geometry.approximate(&aabb).into_iter().map(
        |point_surface| {
            let point_global = surface.geometry.point_from_local(point_surface);

            TriangulationPoint {
                point_surface,
                point_global,
            }
        },
    ))
}

fn triangles(points: &[TriangulationPoint]) -> Vec<[TriangulationPoint; 3]> {
    let mut triangulation = spade::ConstrainedDelaunayTriangulation::<_>::new();

    // We're passing duplicate points to the triangulation here. It doesn't seem
    // to mind though.
    triangulation
        .add_constraint_edges(points.iter().copied(), true)
        .unwrap();

    triangulation
        .inner_faces()
        .map(|triangle| triangle.vertices().map(|vertex| *vertex.data()))
        .collect()
}
