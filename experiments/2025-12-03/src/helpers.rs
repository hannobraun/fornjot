use std::{
    collections::{BTreeSet, VecDeque},
    mem,
};

use fj_math::{Point, Triangle};
use geo::{Contains, Coord, LineString, Polygon};
use spade::Triangulation;

use crate::approx::{ApproxPoint, HalfEdgeApprox};

pub fn approx_face(boundary: &[HalfEdgeApprox]) -> Vec<Triangle<3>> {
    let Some(start) = boundary.first().map(|half_edge| half_edge.start) else {
        return Vec::new();
    };

    let boundary_points =
        boundary.iter().flat_map(|half_edge| half_edge.points());
    let boundary_polygon = polygon(
        boundary
            .iter()
            .flat_map(|half_edge| half_edge.points().map(|point| point.local))
            .chain([start.local]),
    );

    delaunay(boundary_points)
        .into_iter()
        .filter(|triangle| {
            let points = triangle.map(|point| point.local);
            let [x, y] = Triangle::from_points(points)
                .center()
                .coords
                .components
                .map(|s| s.into_f64());

            boundary_polygon.contains(&Coord { x, y })
        })
        .map(|triangle| {
            let [p0, p1, p2] = triangle.map(|point| point.global);
            let triangle = Triangle::from([p0, p1, p2]);

            if !triangle.is_valid() {
                panic!("Expected valid triangle; got: {triangle:?}");
            }

            triangle
        })
        .collect()
}

fn polygon(points: impl IntoIterator<Item = Point<2>>) -> Polygon {
    let mut line_strings = VecDeque::new();
    let mut current_line_string = Vec::new();
    let mut visited_points = BTreeSet::new();

    for point in points {
        if visited_points.contains(&point) {
            line_strings.push_back(mem::take(&mut current_line_string));
            continue;
        }

        let [x, y] = point.coords.components.map(|s| s.into_f64());
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

fn delaunay(
    points: impl IntoIterator<Item = ApproxPoint<2>>,
) -> Vec<[ApproxPoint<2>; 3]> {
    let mut triangulation = spade::ConstrainedDelaunayTriangulation::<_>::new();

    triangulation.add_constraint_edges(points, true).unwrap();

    triangulation
        .inner_faces()
        .map(|triangle| triangle.vertices().map(|vertex| *vertex.data()))
        .collect()
}
