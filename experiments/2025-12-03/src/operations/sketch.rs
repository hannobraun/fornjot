use fj_math::{Point, Scalar};
use itertools::Itertools;

use crate::{
    geometry::{
        curve::{Arc, Curve, Line},
        surface::Plane,
    },
    helpers::approx_face,
    store::{Index, Store},
    topology::{Face, HalfEdge, Vertex},
};

pub struct Sketch {
    start: Point<2>,
    segments: Vec<SketchSegment>,
}

impl Sketch {
    pub fn start_at(start: impl Into<Point<2>>) -> Self {
        let start = start.into();

        Self {
            start,
            segments: Vec::new(),
        }
    }

    pub fn arc_to(
        mut self,
        destination: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        tolerance: impl Into<Scalar>,
    ) -> Self {
        self.segments.push(SketchSegment {
            geometry: SketchSegmentGeometry::Arc {
                destination: destination.into(),
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
            attachment: None,
        });

        self
    }

    pub fn arc_to_at(
        mut self,
        destination: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        tolerance: impl Into<Scalar>,
        vertex: Index<Vertex>,
    ) -> Self {
        self.segments.push(SketchSegment {
            geometry: SketchSegmentGeometry::Arc {
                destination: destination.into(),
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
            attachment: Some(SketchSegmentAttachment::Vertex { vertex }),
        });

        self
    }

    pub fn line_to(mut self, destination: impl Into<Point<2>>) -> Self {
        self.segments.push(SketchSegment {
            geometry: SketchSegmentGeometry::Line {
                destination: destination.into(),
            },
            attachment: None,
        });

        self
    }

    pub fn line_to_at(
        mut self,
        destination: impl Into<Point<2>>,
        vertex: Index<Vertex>,
    ) -> Self {
        let attachment = Some(SketchSegmentAttachment::Vertex { vertex });
        self.segments.push(SketchSegment {
            geometry: SketchSegmentGeometry::Line {
                destination: destination.into(),
            },
            attachment,
        });

        self
    }

    pub fn into_face(
        mut self,
        surface: Plane,
        vertices: &mut Store<Vertex>,
        half_edges: &mut Store<HalfEdge>,
        faces: &mut Store<Face>,
    ) -> Index<Face> {
        let Some(last_segment_index) = self.segments.len().checked_sub(1)
        else {
            panic!("Empty sketches are not supported at this point.");
        };

        let mut positions_and_half_edges_and_approx = Vec::new();

        for i in 0..=last_segment_index {
            let prev_i = i.checked_sub(1).unwrap_or(last_segment_index);
            let next_i = if i == last_segment_index { 0 } else { i + 1 };

            let current = &self.segments[i];
            let prev = &self.segments[prev_i];
            let next = &self.segments[next_i];

            let (half_edge, approx) =
                current.with_curve().to_half_edge_and_approx(
                    prev.with_curve(),
                    next.with_curve(),
                    &surface,
                    half_edges,
                    vertices,
                );

            let destination = match current.geometry {
                SketchSegmentGeometry::Arc { destination, .. } => destination,
                SketchSegmentGeometry::Line { destination } => destination,
            };

            positions_and_half_edges_and_approx.push((
                destination,
                half_edge,
                approx,
            ));
            self.segments[i].attachment =
                Some(SketchSegmentAttachment::HalfEdge { half_edge });
        }

        for (&(_, a, _), &(_, b, _)) in positions_and_half_edges_and_approx
            .iter()
            .circular_tuple_windows()
        {
            assert_eq!(half_edges[a].boundary[1], half_edges[b].boundary[0]);
        }

        let boundary = positions_and_half_edges_and_approx
            .iter()
            .map(|&(_, half_edge, _)| half_edge)
            .collect();

        let approx = approx_face(
            self.start,
            positions_and_half_edges_and_approx,
            vertices,
            half_edges,
        );

        faces.push(Face { boundary, approx })
    }
}

struct SketchSegment {
    pub geometry: SketchSegmentGeometry,
    pub attachment: Option<SketchSegmentAttachment>,
}

impl SketchSegment {
    pub fn with_curve<'r>(&'r self) -> SketchSegmentAndCurve<'r> {
        let curve: Box<dyn Curve> = match self.geometry {
            SketchSegmentGeometry::Arc {
                destination,
                radius,
                tolerance,
            } => Box::new(Arc {
                end: destination,
                radius,
                tolerance,
            }),
            SketchSegmentGeometry::Line { destination } => {
                Box::new(Line { end: destination })
            }
        };

        SketchSegmentAndCurve {
            segment: self,
            curve,
        }
    }

    pub fn to_start_vertex(
        &self,
        position: Point<2>,
        surface: &Plane,
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
}

enum SketchSegmentGeometry {
    Arc {
        destination: Point<2>,
        radius: Scalar,
        tolerance: Scalar,
    },
    Line {
        destination: Point<2>,
    },
}

#[derive(Clone, Copy, Debug)]
enum SketchSegmentAttachment {
    HalfEdge { half_edge: Index<HalfEdge> },
    Vertex { vertex: Index<Vertex> },
}

struct SketchSegmentAndCurve<'r> {
    segment: &'r SketchSegment,
    curve: Box<dyn Curve>,
}

impl SketchSegmentAndCurve<'_> {
    pub fn to_half_edge_and_approx(
        &self,
        prev: SketchSegmentAndCurve,
        next: SketchSegmentAndCurve,
        surface: &Plane,
        half_edges: &mut Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> (Index<HalfEdge>, Vec<Point<2>>) {
        let approx = self.curve.approx(prev.curve.end());

        let boundary = match self.segment.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                // We just assume that the approximation of the sketch segment
                // and the existing approximation of the half-edge match. We
                // should make sure by checking it here.
                return (half_edge, approx);
            }
            Some(SketchSegmentAttachment::Vertex { vertex: v1 }) => {
                let v0 = prev
                    .segment
                    .with_curve()
                    .to_end_vertex(surface, half_edges, vertices);

                [v0, v1]
            }
            None => {
                let v0 = prev
                    .segment
                    .with_curve()
                    .to_end_vertex(surface, half_edges, vertices);
                let v1 = next.segment.to_start_vertex(
                    self.curve.end(),
                    surface,
                    half_edges,
                    vertices,
                );

                [v0, v1]
            }
        };

        let half_edge = half_edges.push(HalfEdge {
            boundary,
            approx: approx
                .iter()
                .copied()
                .map(|local| surface.local_to_global(local))
                .collect(),
        });

        (half_edge, approx)
    }

    pub fn to_end_vertex(
        &self,
        surface: &Plane,
        half_edges: &Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> Index<Vertex> {
        match self.segment.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                let [_, vertex] = half_edges[half_edge].boundary;
                vertex
            }
            Some(SketchSegmentAttachment::Vertex { vertex }) => vertex,
            None => {
                let point = surface.local_to_global(self.curve.end());
                vertices.push(Vertex { point })
            }
        }
    }
}
