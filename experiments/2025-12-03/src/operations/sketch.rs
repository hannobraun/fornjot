use fj_math::Point;
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

    pub fn line_to_with_half_edge(
        mut self,
        position: impl Into<Point<2>>,
        half_edge: Index<HalfEdge>,
    ) -> Sketch {
        self.segments.push(SketchSegment {
            to: position.into(),
            attachment: SketchSegmentAttachment::HalfEdge { half_edge },
        });

        self
    }

    pub fn line_to_vertex(
        self,
        position: impl Into<Point<2>>,
        v2: Index<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Sketch {
        let position = position.into();

        let Some(e01) = self.segments.last().copied() else {
            panic!(
                "Can't push single vertex to sketch, unless at least one \
                half-edge is already available."
            );
        };

        let SketchSegmentAttachment::HalfEdge { half_edge } = e01.attachment;
        let [_, v1] = half_edges[half_edge].boundary;
        let e01 = half_edges.push(HalfEdge { boundary: [v1, v2] });

        self.line_to_with_half_edge(position, e01)
    }

    pub fn close(self, half_edges: &mut Store<HalfEdge>) -> Sketch {
        let [Some(e01), Some(e12)] =
            [self.segments.first(), self.segments.last()]
                .map(|opt| opt.copied())
        else {
            panic!(
                "Can't close sketch, if there's not already at least one \
                half-edge."
            );
        };

        let [v0, _] = {
            let SketchSegmentAttachment::HalfEdge { half_edge } =
                e01.attachment;
            half_edges[half_edge].boundary
        };
        let [_, v2] = {
            let SketchSegmentAttachment::HalfEdge { half_edge } =
                e12.attachment;
            half_edges[half_edge].boundary
        };

        let e20 = half_edges.push(HalfEdge { boundary: [v2, v0] });

        let start = self.start;
        self.line_to_with_half_edge(start, e20)
    }

    pub fn build(
        self,
        vertices: &Store<Vertex>,
        half_edges: &Store<HalfEdge>,
        triangles: &mut Triangles,
        faces: &mut Faces,
    ) -> Index<Face> {
        let boundary = self
            .segments
            .iter()
            .copied()
            .map(|segment| {
                let SketchSegmentAttachment::HalfEdge { half_edge } =
                    segment.attachment;
                half_edge
            })
            .collect::<Vec<_>>();

        for (a, b) in boundary.iter().copied().circular_tuple_windows() {
            let a = half_edges[a];
            let b = half_edges[b];

            assert_eq!(a.boundary[1], b.boundary[0]);
        }

        let delaunay_points = self.segments.iter().map(|segment| {
            let [_, vertex] = {
                let SketchSegmentAttachment::HalfEdge { half_edge } =
                    segment.attachment;
                half_edges[half_edge].boundary
            };
            DelaunayPoint {
                position: segment.to,
                vertex,
            }
        });
        let triangles = delaunay(delaunay_points)
            .into_iter()
            .map(|triangle| {
                let [v0, v1, v2] = triangle.map(|point| point.vertex);
                triangles.push([v0, v1, v2], vertices)
            })
            .collect();

        faces.push(Face {
            boundary,
            triangles,
        })
    }
}

#[derive(Clone, Copy)]
struct SketchSegment {
    pub to: Point<2>,
    pub attachment: SketchSegmentAttachment,
}

#[derive(Clone, Copy)]
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
