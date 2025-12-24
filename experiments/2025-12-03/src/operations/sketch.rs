use std::{
    collections::{BTreeSet, VecDeque},
    mem,
};

use fj_math::{Point, Triangle, Vector};
use geo::{Contains, Coord, LineString, Polygon};
use itertools::Itertools;
use spade::Triangulation;

use crate::{
    objects::{
        geometry::{Geometry, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    store::{Index, Store},
};

pub struct Sketch {
    start: Point<2>,
    segments: Vec<SketchSegment>,
}

impl Sketch {
    pub fn start_at(start: impl Into<Point<2>>) -> Self {
        Self {
            start: start.into(),
            segments: Vec::new(),
        }
    }

    pub fn line_to(mut self, position: impl Into<Point<2>>) -> Self {
        self.segments.push(SketchSegment {
            to: position.into(),
            attachment: None,
        });

        self
    }

    pub fn line_to_with_half_edge(
        mut self,
        position: impl Into<Point<2>>,
        half_edge: Index<HalfEdge>,
    ) -> Self {
        self.segments.push(SketchSegment {
            to: position.into(),
            attachment: Some(SketchSegmentAttachment::HalfEdge { half_edge }),
        });

        self
    }

    pub fn line_to_vertex(
        mut self,
        position: impl Into<Point<2>>,
        vertex: Index<Vertex>,
    ) -> Self {
        self.segments.push(SketchSegment {
            to: position.into(),
            attachment: Some(SketchSegmentAttachment::Vertex { vertex }),
        });

        self
    }

    pub fn into_face(
        mut self,
        surface: Surface,
        geometry: &mut Geometry,
        half_edges: &mut Store<HalfEdge>,
        faces: &mut Faces,
    ) -> Index<Face> {
        let Some(last_segment_index) = self.segments.len().checked_sub(1)
        else {
            panic!("Empty sketches are not supported at this point.");
        };

        let mut positions_and_half_edges = Vec::new();

        for i in 0..=last_segment_index {
            let prev_i = i.checked_sub(1).unwrap_or(last_segment_index);
            let next_i = if i == last_segment_index { 0 } else { i + 1 };

            let current = self.segments[i];
            let prev = self.segments[prev_i];
            let next = self.segments[next_i];

            let half_edge = match current.attachment {
                Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                    half_edge
                }
                Some(SketchSegmentAttachment::Vertex { vertex: v1 }) => {
                    let v0 = match prev.attachment {
                        Some(SketchSegmentAttachment::HalfEdge {
                            half_edge,
                        }) => {
                            let [_, vertex] = half_edges[half_edge].boundary;
                            vertex
                        }
                        Some(SketchSegmentAttachment::Vertex { vertex }) => {
                            vertex
                        }
                        None => {
                            let point = surface.local_to_global(prev.to);
                            geometry.vertices.push(Vertex { point })
                        }
                    };

                    half_edges.push(HalfEdge { boundary: [v0, v1] })
                }
                None => {
                    let v0 = match prev.attachment {
                        Some(SketchSegmentAttachment::HalfEdge {
                            half_edge,
                        }) => {
                            let [_, vertex] = half_edges[half_edge].boundary;
                            vertex
                        }
                        Some(SketchSegmentAttachment::Vertex { vertex }) => {
                            vertex
                        }
                        None => {
                            let point = surface.local_to_global(prev.to);
                            geometry.vertices.push(Vertex { point })
                        }
                    };
                    let v1 = match next.attachment {
                        Some(SketchSegmentAttachment::HalfEdge {
                            half_edge,
                        }) => {
                            let [vertex, _] = half_edges[half_edge].boundary;
                            vertex
                        }
                        Some(SketchSegmentAttachment::Vertex { vertex: _ })
                        | None => {
                            let point = surface.local_to_global(current.to);
                            geometry.vertices.push(Vertex { point })
                        }
                    };

                    half_edges.push(HalfEdge { boundary: [v0, v1] })
                }
            };

            positions_and_half_edges.push((current.to, half_edge));
            self.segments[i].attachment =
                Some(SketchSegmentAttachment::HalfEdge { half_edge });
        }

        for ((_, a), (_, b)) in positions_and_half_edges
            .iter()
            .copied()
            .circular_tuple_windows()
        {
            assert_eq!(half_edges[a].boundary[1], half_edges[b].boundary[0]);
        }

        let delaunay_points = positions_and_half_edges.iter().copied().map(
            |(local, half_edge)| {
                let [_, vertex] = half_edges[half_edge].boundary;
                let global = geometry.vertices[vertex].point;

                DelaunayPoint { local, global }
            },
        );
        let polygon = polygon(
            [self.start].into_iter().chain(
                positions_and_half_edges
                    .iter()
                    .copied()
                    .map(|(position, _)| position),
            ),
        );

        let triangles = delaunay(delaunay_points)
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
                geometry.triangles.push([p0, p1, p2], &geometry.points)
            })
            .collect();

        faces.push(Face {
            boundary: positions_and_half_edges
                .into_iter()
                .map(|(_, half_edge)| half_edge)
                .collect(),
            triangles,
        })
    }
}

pub struct Surface {
    pub origin: Point<3>,
    pub axes: [Vector<3>; 2],
}

impl Surface {
    pub fn local_to_global(&self, local: Point<2>) -> Point<3> {
        let [u, v] = local.coords.components;
        let [axis_u, axis_v] = self.axes;

        self.origin + axis_u * u + axis_v * v
    }
}

#[derive(Clone, Copy, Debug)]
struct SketchSegment {
    pub to: Point<2>,
    pub attachment: Option<SketchSegmentAttachment>,
}

#[derive(Clone, Copy, Debug)]
enum SketchSegmentAttachment {
    HalfEdge { half_edge: Index<HalfEdge> },
    Vertex { vertex: Index<Vertex> },
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

#[derive(Clone, Copy)]
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
