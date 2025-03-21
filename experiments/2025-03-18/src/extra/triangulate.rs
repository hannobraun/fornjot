use std::{
    collections::{BTreeSet, VecDeque},
    mem,
};

use geo::{Contains, Coord, LineString, Polygon};
use spade::Triangulation;

use crate::{
    geometry::{MeshTriangle, TriMesh, Triangle},
    math::Point,
    topology::face::Face,
};

pub fn triangulate(face: &Face) -> TriMesh {
    let points = points(face);
    let triangles = triangles(&points);

    let polygon = polygon(&points);
    let triangles_in_face = triangles
        .into_iter()
        .filter(|triangle| {
            let points = triangle.map(|point| point.point_surface);
            let triangle = Triangle { points };

            let [x, y] =
                triangle.center().coords.components.map(|s| s.into_f64());
            polygon.contains(&Coord { x, y })
        })
        .map(|triangle| {
            let points = triangle.map(|point| point.point_vertex);
            MeshTriangle {
                inner: Triangle { points },
                is_internal: face.is_internal,
            }
        });

    let mut mesh = TriMesh::new();
    mesh.triangles.extend(triangles_in_face);

    mesh
}

fn points(face: &Face) -> Vec<TriangulationPoint> {
    face.half_edges
        .iter()
        .map(|half_edge| {
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
                face.surface.geometry.project_point(half_edge.start.point);

            TriangulationPoint {
                point_surface,
                point_vertex: half_edge.start.point,
            }
        })
        .collect()
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

fn polygon(points: &[TriangulationPoint]) -> Polygon {
    // This is a placeholder implementation that is probably not well-tested and
    // probably doesn't support polygons with multiple holes.

    let mut line_strings = VecDeque::new();
    let mut current_line_string = Vec::new();
    let mut visited_points = BTreeSet::new();

    for point in points {
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

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct TriangulationPoint {
    point_surface: Point<2>,
    point_vertex: Point<3>,
}

impl spade::HasPosition for TriangulationPoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.point_surface.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
