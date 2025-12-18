use fj_math::{Point, Vector};
use itertools::Itertools;
use spade::Triangulation;

use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    store::{Index, Store},
};

pub struct Sketch {
    start: Point<2>,
    segments: Vec<SketchSegment>,
}

impl Sketch {
    pub fn start_at(position: impl Into<Point<2>>) -> Self {
        Self {
            start: position.into(),
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

    pub fn close(mut self) -> Self {
        self.segments.push(SketchSegment {
            to: self.start,
            attachment: None,
        });

        self
    }

    pub fn into_face(
        mut self,
        surface: Surface,
        vertices: &mut Store<Vertex>,
        triangles: &mut Triangles,
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
                None => {
                    let v0 = match prev.attachment {
                        Some(SketchSegmentAttachment::HalfEdge {
                            half_edge,
                        }) => {
                            let [_, vertex] = half_edges[half_edge].boundary;
                            vertex
                        }
                        None => {
                            let position = surface.local_to_global(prev.to);
                            vertices.push(Vertex { position })
                        }
                    };
                    let v1 = match next.attachment {
                        Some(SketchSegmentAttachment::HalfEdge {
                            half_edge,
                        }) => {
                            let [vertex, _] = half_edges[half_edge].boundary;
                            vertex
                        }
                        None => {
                            let position = surface.local_to_global(current.to);
                            vertices.push(Vertex { position })
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
            |(position, half_edge)| {
                let [_, vertex] = half_edges[half_edge].boundary;
                DelaunayPoint { position, vertex }
            },
        );
        let triangles = delaunay(delaunay_points)
            .into_iter()
            .map(|triangle| {
                let [v0, v1, v2] = triangle.map(|point| point.vertex);
                triangles.push([v0, v1, v2], vertices)
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
    pub position: Point<2>,
    pub vertex: Index<Vertex>,
}

impl spade::HasPosition for DelaunayPoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.position.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
