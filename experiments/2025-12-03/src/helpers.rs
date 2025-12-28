use std::{
    collections::{BTreeSet, VecDeque},
    mem,
};

use fj_math::{Point, Triangle};
use geo::{Contains, Coord, LineString, Polygon};
use spade::Triangulation;

use crate::{
    objects::topology::{HalfEdge, Vertex},
    store::{Index, Store},
};

pub fn approx_face(
    start: impl Into<Point<2>>,
    positions_and_half_edges_and_approx: Vec<(
        Point<2>,
        Index<HalfEdge>,
        Vec<Point<2>>,
    )>,
    vertices: &Store<Vertex>,
    half_edges: &Store<HalfEdge>,
) -> Vec<Triangle<3>> {
    let start = start.into();

    let polygon = polygon([start].into_iter().chain(
        positions_and_half_edges_and_approx.iter().flat_map(
            |(position, _, approx)| approx.iter().chain([position]).copied(),
        ),
    ));

    let points = positions_and_half_edges_and_approx.into_iter().flat_map(
        |(local, half_edge, approx)| {
            let half_edge = &half_edges[half_edge];

            assert_eq!(half_edge.approx.len(), approx.len());

            let points_from_approx = approx
                .into_iter()
                .zip(half_edge.approx.iter().copied())
                .map(|(local, global)| DelaunayPoint { local, global });
            let point_from_vertex = {
                let [_, vertex] = half_edge.boundary;
                let global = vertices[vertex].point;

                DelaunayPoint { local, global }
            };

            points_from_approx.into_iter().chain([point_from_vertex])
        },
    );

    delaunay(points)
        .into_iter()
        .filter(|triangle| {
            let points = triangle.map(|point| point.local);
            let [x, y] = Triangle::from_points(points)
                .center()
                .coords
                .components
                .map(|s| s.into_f64());

            polygon.contains(&Coord { x, y })
        })
        .map(|triangle| {
            let [p0, p1, p2] = triangle.map(|point| point.global);
            Triangle::from([p0, p1, p2])
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
    points: impl IntoIterator<Item = DelaunayPoint>,
) -> Vec<[DelaunayPoint; 3]> {
    let mut triangulation = spade::ConstrainedDelaunayTriangulation::<_>::new();

    triangulation.add_constraint_edges(points, true).unwrap();

    triangulation
        .inner_faces()
        .map(|triangle| triangle.vertices().map(|vertex| *vertex.data()))
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct DelaunayPoint {
    pub local: Point<2>,
    pub global: Point<3>,
}

impl spade::HasPosition for DelaunayPoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.local.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
