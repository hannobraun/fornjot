use std::{
    collections::{BTreeSet, VecDeque},
    f64::consts::PI,
    mem,
};

use fj_interop::{Color, MeshTriangle, Tolerance, TriMesh};
use fj_math::{Aabb, Point, Triangle};
use geo::{Contains, Coord, LineString, Polygon};

use crate::{
    approx::{face::FaceApprox, half_edge::HalfEdgeApprox},
    extra::triangulate::{delaunay::triangles, surface::SurfaceMesh},
    topology::face::Face,
};

use super::TriangulationPoint;

pub fn triangulate_face(
    face: &Face,
    tolerance: impl Into<Tolerance>,
) -> TriMesh {
    let tolerance = tolerance.into();

    let surface_mesh = {
        // This happens to be big enough for the current model. But
        // eventually, we need a solution here that works for _any_ model.
        let size = PI;

        let boundary = Aabb {
            min: Point::from([-size, -size]),
            max: Point::from([size, size]),
        };

        SurfaceMesh::new(&face.surface, &boundary, tolerance)
    };

    let points_from_half_edges =
        half_edges_to_points(face, &surface_mesh, tolerance);

    let polygon_from_half_edges =
        polygon_from_half_edges(&points_from_half_edges);

    let surface_points = surface_mesh.points.into_iter().filter(|point| {
        polygon_from_half_edges.contains(&Coord {
            x: point.point_surface.u.into_f64(),
            y: point.point_surface.v.into_f64(),
        })
    });

    let triangles = triangles(points_from_half_edges, surface_points)
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
    surface: &SurfaceMesh,
    tolerance: impl Into<Tolerance>,
) -> FaceApprox {
    let tolerance = tolerance.into();

    face.half_edges_with_end_vertex()
        .flat_map(|half_edge_with_end_vertex| {
            HalfEdgeApprox::from_half_edge_with_end_vertex(
                half_edge_with_end_vertex,
                tolerance,
            )
            .points
        })
        .map(|point_global| {
            // Here, we project a 3D point (from the vertex) into the face's
            // surface, creating a 2D point. Through the surface, this 2D point
            // has a position in 3D space.
            //
            // But this position isn't necessarily going to be the same as the
            // position of the original 3D point, due to numerical inaccuracy.
            //
            // This doesn't matter. Neither does the fact, that other faces
            // might share the same vertices and project them into their own
            // surfaces, creating more redundancy.
            //
            // The reason that it doesn't, is that we're using the projected 2D
            // points _only_ for this local triangulation. Once that tells us
            // how the different 3D points must connect, we use the original 3D
            // points to build those triangles. We never convert the 2D points
            // back into 3D.
            let point_surface = surface.project_point(point_global, tolerance);

            TriangulationPoint {
                point_surface,
                point_global,
            }
        })
        .collect()
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
