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

pub struct Sketch<const SIZE: usize> {
    segments: [SketchSegment; SIZE],
}

impl Sketch<0> {
    pub fn new() -> Self {
        Self { segments: [] }
    }

    pub fn push_half_edge(
        self,
        to: impl Into<Point<2>>,
        e01: Index<HalfEdge>,
    ) -> Sketch<1> {
        let to = to.into();

        let [] = self.segments.map(|segment| segment.half_edge);

        Sketch {
            segments: [e01].map(|half_edge| SketchSegment { to, half_edge }),
        }
    }
}

impl Sketch<1> {
    pub fn push_half_edge(
        self,
        to: impl Into<Point<2>>,
        e12: Index<HalfEdge>,
    ) -> Sketch<2> {
        let to = to.into();

        let [e01] = self.segments;

        Sketch {
            segments: [e01, SketchSegment { to, half_edge: e12 }],
        }
    }

    pub fn push_vertex(
        self,
        position: impl Into<Point<2>>,
        v2: Index<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Sketch<2> {
        let position = position.into();

        let Some(e01) = self.segments.last().copied() else {
            panic!(
                "Can't push single vertex to sketch, unless at least one \
                half-edge is already available."
            );
        };

        let [_, v1] = half_edges[e01.half_edge].boundary;
        let e12 = half_edges.push(HalfEdge { boundary: [v1, v2] });

        self.push_half_edge(position, e12)
    }
}

impl Sketch<2> {
    pub fn push_half_edge(
        self,
        to: impl Into<Point<2>>,
        e23: Index<HalfEdge>,
    ) -> Sketch<3> {
        let to = to.into();

        let [e01, e12] = self.segments;

        Sketch {
            segments: [e01, e12, SketchSegment { to, half_edge: e23 }],
        }
    }

    pub fn push_vertex(
        self,
        position: impl Into<Point<2>>,
        v3: Index<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Sketch<3> {
        let position = position.into();

        let [e01, e12] = self.segments;

        let [_, v2] = half_edges[e12.half_edge].boundary;
        let e23 = half_edges.push(HalfEdge { boundary: [v2, v3] });

        Sketch {
            segments: [
                e01,
                e12,
                SketchSegment {
                    to: position,
                    half_edge: e23,
                },
            ],
        }
    }
}

impl Sketch<3> {
    pub fn close_with_half_edge(
        self,
        half_edges: &mut Store<HalfEdge>,
    ) -> Sketch<4> {
        let [Some(e01), Some(e12)] =
            [self.segments.first(), self.segments.last()]
                .map(|opt| opt.copied())
        else {
            panic!(
                "Can't close sketch, if there's not already at least one \
                half-edge."
            );
        };

        let [v0, _] = half_edges[e01.half_edge].boundary;
        let [_, v2] = half_edges[e12.half_edge].boundary;

        let e20 = half_edges.push(HalfEdge { boundary: [v2, v0] });
        self.push_half_edge([0., 0.], e20)
    }

    pub fn push_half_edge(
        self,
        to: impl Into<Point<2>>,
        e30: Index<HalfEdge>,
    ) -> Sketch<4> {
        let to = to.into();

        let [e01, e12, e23] = self.segments;

        Sketch {
            segments: [e01, e12, e23, SketchSegment { to, half_edge: e30 }],
        }
    }
}

impl Sketch<4> {
    pub fn build(
        self,
        vertices: &Store<Vertex>,
        half_edges: &Store<HalfEdge>,
        triangles: &mut Triangles,
        faces: &mut Faces,
    ) -> Index<Face> {
        for (a, b) in self.segments.iter().circular_tuple_windows() {
            let [_, a] = half_edges[a.half_edge].boundary;
            let [b, _] = half_edges[b.half_edge].boundary;

            assert_eq!(a, b);
        }

        let delaunay_points = self.segments.iter().map(|segment| {
            let [_, vertex] = half_edges[segment.half_edge].boundary;
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
            boundary: self
                .segments
                .into_iter()
                .map(|segment| segment.half_edge)
                .collect(),
            triangles,
        })
    }
}

#[derive(Clone, Copy)]
struct SketchSegment {
    pub to: Point<2>,
    pub half_edge: Index<HalfEdge>,
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
