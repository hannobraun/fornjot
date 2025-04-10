use std::{
    collections::{BTreeSet, VecDeque},
    mem,
};

use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::{Point, Triangle};
use geo::{Contains, Coord, LineString, Polygon};
use spade::Triangulation;

use crate::topology::{
    face::{Face, HalfEdgeWithEndVertex},
    surface::Surface,
};

pub fn triangulate(face: &Face) -> TriMesh {
    let mut points_from_half_edges = Vec::new();
    half_edges_to_points(face, &mut points_from_half_edges);

    let polygon_from_half_edges =
        polygon_from_half_edges(&points_from_half_edges);

    let mut all_points = points_from_half_edges;
    points_from_surface(
        &face.surface,
        &polygon_from_half_edges,
        &mut all_points,
    );

    let triangles_in_face = triangles(&all_points)
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
    mesh.triangles.extend(triangles_in_face);

    mesh
}

fn half_edges_to_points(face: &Face, target: &mut Vec<TriangulationPoint>) {
    target.extend(
        face.half_edges_with_end_vertex()
            .map(approximate_half_edge)
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

fn approximate_half_edge(
    HalfEdgeWithEndVertex {
        half_edge,
        end_vertex: _,
    }: HalfEdgeWithEndVertex,
) -> Point<3> {
    let start = &half_edge.start;
    start.point
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
    target.extend(surface.geometry.approximate(boundary).into_iter().map(
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

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct TriangulationPoint {
    point_surface: Point<2>,
    point_global: Point<3>,
}

impl spade::HasPosition for TriangulationPoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.point_surface.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
