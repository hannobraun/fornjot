use fj_math::{Circle, Point, Scalar, Vector};
use itertools::Itertools;

use crate::{
    geometry::surface::Plane,
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
        radius: impl Into<Scalar>,
        tolerance: impl Into<Scalar>,
        to: impl Into<Point<2>>,
    ) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Arc {
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
            attachment: None,
        });

        self
    }

    pub fn arc_to_at(
        mut self,
        radius: impl Into<Scalar>,
        tolerance: impl Into<Scalar>,
        to: impl Into<Point<2>>,
        at: Index<Vertex>,
    ) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Arc {
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
            attachment: Some(SketchSegmentAttachment::Vertex { vertex: at }),
        });

        self
    }

    pub fn line_to(mut self, to: impl Into<Point<2>>) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Line,
            attachment: None,
        });

        self
    }

    pub fn line_to_at(
        mut self,
        to: impl Into<Point<2>>,
        at: Index<Vertex>,
    ) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Line,
            attachment: Some(SketchSegmentAttachment::Vertex { vertex: at }),
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
            panic!("Empty sketches are not supported yet.");
        };

        let mut positions_and_half_edges_and_approx = Vec::new();

        for i in 0..=last_segment_index {
            let prev_i = i.checked_sub(1).unwrap_or(last_segment_index);
            let next_i = if i == last_segment_index { 0 } else { i + 1 };

            let current = &self.segments[i];
            let prev = &self.segments[prev_i];
            let next = &self.segments[next_i];

            let (half_edge, approx) = current.to_half_edge_and_approx(
                prev, next, &surface, half_edges, vertices,
            );

            positions_and_half_edges_and_approx.push((
                current.end,
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

#[derive(Clone, Copy)]
struct SketchSegment {
    pub end: Point<2>,
    pub geometry: SketchSegmentGeometry,
    pub attachment: Option<SketchSegmentAttachment>,
}

impl SketchSegment {
    pub fn to_half_edge_and_approx(
        self,
        prev: &SketchSegment,
        next: &SketchSegment,
        surface: &Plane,
        half_edges: &mut Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> (Index<HalfEdge>, Vec<Point<2>>) {
        let approx = match self.geometry {
            SketchSegmentGeometry::Arc { radius, tolerance } => {
                let start = prev.end;
                let start_to_end = self.end - start;

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
                        // TASK: Fix.
                        panic!(
                            "Radius of arc (`{radius}`) is too small: Must be \
                            at least half the distance between start \
                            (`{start:?}`) and end (`{to:?}`) points, or the \
                            arc is not possible.",
                            to = start_to_end,
                        );
                    }

                    b_squared.sqrt()
                };

                let center = start
                    + start_to_end * 0.5
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

                let start_local = circle.point_to_circle_coords(start);
                let end_local =
                    circle.point_to_circle_coords(start + start_to_end);

                let mut approx = Vec::new();

                let mut point = start_local + increment;
                while point < end_local {
                    approx.push(circle.point_from_circle_coords(point));
                    point += increment;
                }

                approx
            }
            SketchSegmentGeometry::Line => Vec::new(),
        };

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
                let v1 = next
                    .to_start_vertex(self.end, surface, half_edges, vertices);

                [v0, v1]
            }
        };

        let half_edge = half_edges.push(HalfEdge {
            boundary,
            approx: approx
                .iter()
                .copied()
                .map(|local| surface.local_point_to_global(local))
                .collect(),
        });

        (half_edge, approx)
    }

    pub fn to_start_vertex(
        self,
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
                let point = surface.local_point_to_global(position);
                vertices.push(Vertex { point })
            }
        }
    }

    pub fn to_end_vertex(
        self,
        surface: &Plane,
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
                let point = surface.local_point_to_global(self.end);
                vertices.push(Vertex { point })
            }
        }
    }
}

#[derive(Clone, Copy)]
enum SketchSegmentGeometry {
    Arc { radius: Scalar, tolerance: Scalar },
    Line,
}

#[derive(Clone, Copy, Debug)]
enum SketchSegmentAttachment {
    HalfEdge { half_edge: Index<HalfEdge> },
    Vertex { vertex: Index<Vertex> },
}
