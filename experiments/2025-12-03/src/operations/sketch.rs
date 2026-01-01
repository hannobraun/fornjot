use fj_math::{Circle, Point, Scalar, Vector};
use itertools::Itertools;

use crate::{
    geometry::{Arc, Line, Surface},
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
            attachment: None,
            geometry: SketchSegmentGeometry::Arc(Arc {
                to: position.into(),
                radius: radius.into(),
                tolerance: tolerance.into(),
            }),
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
            attachment: Some(SketchSegmentAttachment::Vertex { vertex }),
            geometry: SketchSegmentGeometry::Arc(Arc {
                to: position.into(),
                radius: radius.into(),
                tolerance: tolerance.into(),
            }),
        });

        self
    }

    pub fn line_to(mut self, position: impl Into<Point<2>>) -> Self {
        self.segments.push(SketchSegment {
            attachment: None,
            geometry: SketchSegmentGeometry::Line(Line {
                to: position.into(),
            }),
        });

        self
    }

    pub fn line_to_vertex(
        mut self,
        position: impl Into<Point<2>>,
        vertex: Index<Vertex>,
    ) -> Self {
        self.segments.push(SketchSegment {
            attachment: Some(SketchSegmentAttachment::Vertex { vertex }),
            geometry: SketchSegmentGeometry::Line(Line {
                to: position.into(),
            }),
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

        let mut positions_and_half_edges_and_approx = Vec::new();

        for i in 0..=last_segment_index {
            let prev_i = i.checked_sub(1).unwrap_or(last_segment_index);
            let next_i = if i == last_segment_index { 0 } else { i + 1 };

            let current = &self.segments[i];
            let prev = &self.segments[prev_i];
            let next = self.segments[next_i];

            let (half_edge, approx) = current.to_half_edge_and_approx(
                prev, &next, &surface, half_edges, vertices,
            );

            positions_and_half_edges_and_approx.push((
                current.geometry.to(),
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

#[derive(Clone, Copy, Debug)]
struct SketchSegment {
    pub attachment: Option<SketchSegmentAttachment>,
    pub geometry: SketchSegmentGeometry,
}

impl SketchSegment {
    pub fn to_half_edge_and_approx(
        self,
        prev: &SketchSegment,
        next: &SketchSegment,
        surface: &Surface,
        half_edges: &mut Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> (Index<HalfEdge>, Vec<Point<2>>) {
        let approx = self.geometry.approx(prev.geometry.to());

        let boundary = match self.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                // We just assume that the approximation of the sketch segment
                // and the existing approximation of the half-edge match. We
                // should make sure by checking it here.
                return (half_edge, approx);
            }
            Some(SketchSegmentAttachment::Vertex { vertex: v1 }) => {
                let v0 = prev.to_end_vertex(surface, half_edges, vertices);

                [v0, v1]
            }
            None => {
                let v0 = prev.to_end_vertex(surface, half_edges, vertices);
                let v1 = next.to_start_vertex(
                    self.geometry.to(),
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
                let point = surface.local_to_global(self.geometry.to());
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
    Arc(Arc),
    Line(Line),
}

impl SketchSegmentGeometry {
    pub fn to(&self) -> Point<2> {
        match *self {
            Self::Arc(Arc { to, .. }) => to,
            Self::Line(Line { to }) => to,
        }
    }

    pub fn approx(&self, start: Point<2>) -> Vec<Point<2>> {
        match *self {
            SketchSegmentGeometry::Arc(Arc {
                to,
                radius,
                tolerance,
            }) => {
                let start_to_end = to - start;
                let midpoint = start + start_to_end * 0.5;

                let midpoint_towards_center =
                    start_to_end.to_perpendicular().normalize()
                        * radius.sign().to_scalar();

                let distance_from_midpoint_to_center = {
                    // We're computing the required distance from a right
                    // triangle:
                    //
                    // - `a` (leg): `midpoint` to `end`
                    // - `b` (leg): `midpoint` to circle center (the distance
                    //   we're looking for)
                    // - `c` (hypotenuse): `end` to circle center (which is
                    //   `radius`)

                    let a = start_to_end.magnitude() / 2.;
                    let c = radius;

                    let b_squared = c * c - a * a;

                    if b_squared < Scalar::ZERO {
                        panic!(
                            "Radius of arc (`{radius}`) is too small: Must be \
                            at least half the distance between start \
                            (`{start:?}`) and end (`{to:?}`) points, or the \
                            arc is not possible."
                        );
                    }

                    b_squared.sqrt()
                };

                let center = midpoint
                    + midpoint_towards_center
                        * distance_from_midpoint_to_center;

                // This only works if `surface` is a plane, which checks out for
                // now.
                let circle = {
                    let a = start;
                    let b = center + (a - center).to_perpendicular();

                    Circle::new(center, a - center, b - center)
                };

                let num_vertices_to_approx_full_circle = Scalar::max(
                    Scalar::PI / (Scalar::ONE - (tolerance / radius)).acos(),
                    3.,
                )
                .ceil();

                let increment = Vector::from([
                    Scalar::TAU / num_vertices_to_approx_full_circle
                ]);

                let start = circle.point_to_circle_coords(start);
                let end = circle.point_to_circle_coords(to);

                let mut approx = Vec::new();

                let mut point = start + increment;
                while point < end {
                    approx.push(circle.point_from_circle_coords(point));
                    point += increment;
                }

                approx
            }
            SketchSegmentGeometry::Line(Line { to: _ }) => Vec::new(),
        }
    }
}
