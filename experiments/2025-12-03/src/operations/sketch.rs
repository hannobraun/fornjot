use std::{
    collections::{BTreeSet, VecDeque},
    mem,
};

use fj_math::{Point, Scalar, Triangle, Vector};
use geo::{Contains, Coord, LineString, Polygon};
use itertools::Itertools;
use spade::Triangulation;

use crate::{
    objects::topology::{Face, HalfEdge, Vertex},
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

    pub fn arc_to(
        mut self,
        position: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        tolerance: impl Into<Scalar>,
    ) -> Self {
        self.segments.push(SketchSegment {
            to: position.into(),
            attachment: None,
            geometry: SketchSegmentGeometry::Arc {
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
        });

        self
    }

    pub fn arc_to_vertex(
        mut self,
        position: impl Into<Point<2>>,
        vertex: Index<Vertex>,
        radius: impl Into<Scalar>,
        tolerance: impl Into<Scalar>,
    ) -> Self {
        self.segments.push(SketchSegment {
            to: position.into(),
            attachment: Some(SketchSegmentAttachment::Vertex { vertex }),
            geometry: SketchSegmentGeometry::Arc {
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
        });

        self
    }

    pub fn line_to(mut self, position: impl Into<Point<2>>) -> Self {
        self.segments.push(SketchSegment {
            to: position.into(),
            attachment: None,
            geometry: SketchSegmentGeometry::Line,
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
            geometry: SketchSegmentGeometry::Line,
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
            geometry: SketchSegmentGeometry::Line,
        });

        self
    }

    pub fn into_face(
        mut self,
        surface: Surface,
        vertices: &mut Store<Vertex>,
        half_edges: &mut Store<HalfEdge>,
        faces: &mut Store<Face>,
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

            let half_edge = current
                .to_half_edge(prev, next, &surface, half_edges, vertices);

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
                let global = vertices[vertex].point;

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

        let approx = delaunay(delaunay_points)
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
            .collect();

        faces.push(Face {
            boundary: positions_and_half_edges
                .into_iter()
                .map(|(_, half_edge)| half_edge)
                .collect(),
            approx,
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
    pub geometry: SketchSegmentGeometry,
}

impl SketchSegment {
    pub fn to_half_edge(
        self,
        prev: SketchSegment,
        next: SketchSegment,
        surface: &Surface,
        half_edges: &mut Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> Index<HalfEdge> {
        let boundary = match self.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                return half_edge;
            }
            Some(SketchSegmentAttachment::Vertex { vertex: v1 }) => {
                let v0 = prev.to_end_vertex(surface, half_edges, vertices);

                [v0, v1]
            }
            None => {
                let v0 = prev.to_end_vertex(surface, half_edges, vertices);
                let v1 = next
                    .to_start_vertex(self.to, surface, half_edges, vertices);

                [v0, v1]
            }
        };

        let approx = self.geometry.approx(prev.to, self.to, surface);

        half_edges.push(HalfEdge {
            boundary,
            approx: approx
                .into_iter()
                .map(|local| surface.local_to_global(local))
                .collect(),
        })
    }

    pub fn to_start_vertex(
        self,
        position: Point<2>,
        surface: &Surface,
        half_edges: &Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> Index<Vertex> {
        match self.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                let [vertex, _] = half_edges[half_edge].boundary;
                vertex
            }
            Some(SketchSegmentAttachment::Vertex { vertex: _ }) | None => {
                let point = surface.local_to_global(position);
                vertices.push(Vertex { point })
            }
        }
    }

    pub fn to_end_vertex(
        self,
        surface: &Surface,
        half_edges: &Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> Index<Vertex> {
        match self.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                let [_, vertex] = half_edges[half_edge].boundary;
                vertex
            }
            Some(SketchSegmentAttachment::Vertex { vertex }) => vertex,
            None => {
                let point = surface.local_to_global(self.to);
                vertices.push(Vertex { point })
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SketchSegmentAttachment {
    HalfEdge { half_edge: Index<HalfEdge> },
    Vertex { vertex: Index<Vertex> },
}

#[derive(Clone, Copy, Debug)]
enum SketchSegmentGeometry {
    Arc { radius: Scalar, tolerance: Scalar },
    Line,
}

impl SketchSegmentGeometry {
    pub fn approx(
        &self,
        start: Point<2>,
        end: Point<2>,
        surface: &Surface,
    ) -> Vec<Point<2>> {
        let _ = start;
        let _ = end;
        let _ = surface;

        match *self {
            SketchSegmentGeometry::Arc { radius, tolerance } => {
                let _ = radius;
                let _ = tolerance;

                Vec::new()
            }
            SketchSegmentGeometry::Line => Vec::new(),
        }
    }
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
