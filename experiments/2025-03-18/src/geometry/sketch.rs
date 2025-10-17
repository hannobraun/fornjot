use std::collections::{BTreeMap, BTreeSet};

use fj_interop::Tolerance;
use fj_math::{Point, Scalar};
use itertools::Itertools;

use crate::{
    handle::Handle,
    topology::{
        curve::Curve, face::Face, half_edge::HalfEdge, surface::Surface,
        vertex::Vertex,
    },
};

use super::{Circle, CurveAnchored};

pub struct Sketch {
    segments: Vec<SketchSegment>,
}

impl Sketch {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn arc_from(
        mut self,
        start: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
    ) -> Self {
        let start = start.into();
        let radius = radius.into();

        self.segments.push(SketchSegment::Arc { start, radius });

        self
    }

    pub fn line_from(mut self, start: impl Into<Point<2>>) -> Self {
        let start = start.into();
        self.segments.push(SketchSegment::Line { start });
        self
    }

    pub fn to_face(
        &self,
        surface: Handle<Surface>,
        tolerance: Tolerance,
    ) -> Face {
        let vertices = SegmentsWithStartVertex::new(&self.segments, &surface);
        let half_edges = make_half_edges(&vertices, &surface);

        // This is doing some checks, to make sure that the half-edges have been
        // connected correctly. There are other ways to do this. For now, this
        // is probably the best one, based on the following considerations:
        //
        // 1. These checks could also be done in a unit test, but doing it here
        //    provides a stronger guarantee. It makes sure that the invariants
        //    are true for all data that comes through here, not just a simple
        //    example that a unit test would construct.
        // 2. The invariants that are checked here should, in a generalized
        //    form, be true for all faces, not just the one constructed here.
        //    But so far, there's no infrastructure for this kind of validation.
        //
        // Long-term, such validation infrastructure should exist, and then we
        // can generalize these checks and move them there.
        check_that_half_edges_are_connected(&half_edges, tolerance);

        Face::new(surface, half_edges, false)
    }
}

#[derive(Clone, Copy)]
enum SketchSegment {
    Arc { start: Point<2>, radius: Scalar },
    Line { start: Point<2> },
}

impl SketchSegment {
    fn start(&self) -> &Point<2> {
        match self {
            SketchSegment::Arc { start, .. } => start,
            SketchSegment::Line { start } => start,
        }
    }
}

struct SegmentsWithStartVertex {
    segments_with_start_vertex: Vec<(SketchSegment, Handle<Vertex>)>,
    coincident_vertices: BTreeSet<Handle<Vertex>>,
}

impl SegmentsWithStartVertex {
    fn new(segments: &[SketchSegment], surface: &Handle<Surface>) -> Self {
        let mut vertices_by_local_point: BTreeMap<_, Vec<_>> = BTreeMap::new();
        let mut coincident_vertices = BTreeSet::new();

        let segments_with_start_vertex = segments
            .iter()
            .copied()
            .map(|segment| {
                let point_local = *segment.start();
                let point_global =
                    surface.geometry.point_from_local(point_local);

                let vertex = Handle::new(Vertex::new(point_global));

                vertices_by_local_point
                    .entry(point_local)
                    .or_default()
                    .push(vertex.clone());

                (segment, vertex)
            })
            .collect::<Vec<_>>();

        for vertices in vertices_by_local_point.into_values() {
            if vertices.len() > 1 {
                coincident_vertices.extend(vertices);
            }
        }

        SegmentsWithStartVertex {
            segments_with_start_vertex,
            coincident_vertices,
        }
    }

    fn iter(
        &self,
    ) -> impl Iterator<Item = ([SegmentWithStartVertex; 2], bool)> {
        self.segments_with_start_vertex
            .iter()
            .cloned()
            .circular_tuple_windows()
            .map(|((segment, start), (next_segment, end))| {
                let [start_is_coincident, end_is_coincident] = [&start, &end]
                    .map(|vertex| self.coincident_vertices.contains(vertex));
                let is_internal = start_is_coincident && end_is_coincident;

                (
                    [
                        SegmentWithStartVertex { segment, start },
                        SegmentWithStartVertex {
                            segment: next_segment,
                            start: end,
                        },
                    ],
                    is_internal,
                )
            })
    }
}

struct SegmentWithStartVertex {
    segment: SketchSegment,
    start: Handle<Vertex>,
}

fn make_half_edges(
    vertices: &SegmentsWithStartVertex,
    surface: &Handle<Surface>,
) -> Vec<Handle<HalfEdge>> {
    vertices
        .iter()
        .map(|([segment, next_segment], is_internal)| {
            let curve = match segment.segment {
                SketchSegment::Arc { start, radius } => {
                    let end = next_segment.segment.start();

                    let start_to_end = end - start;
                    let midpoint = start + start_to_end * 0.5;

                    let midpoint_towards_center =
                        start_to_end.to_perpendicular().normalize()
                            * radius.sign().to_scalar();

                    let distance_from_midpoint_to_center = {
                        // We're computing the required distance from a right
                        // triangle:
                        // - `a` (leg): `midpoint` to `end`
                        // - `b` (leg): `midpoint` to circle center (the
                        //   distance we're looking for)
                        // - `c` (hypotenuse): `end` to circle center (which is
                        //   `radius`)

                        let a = start_to_end.magnitude() / 2.;
                        let c = radius;

                        let b_squared = c * c - a * a;

                        if b_squared < Scalar::ZERO {
                            panic!(
                                "Radius of arc (`{radius}`) is too small: Must \
                                be at least half the distance between start \
                                (`{start:?}`) and end (`{end:?}`) points, or \
                                the arc is not possible."
                            );
                        }

                        b_squared.sqrt()
                    };

                    let center = midpoint
                        + midpoint_towards_center
                            * distance_from_midpoint_to_center;

                    // This only works if `surface` is a plane, which checks out
                    // for now.
                    let (origin, circle) = {
                        let a = start;
                        let b = center + (a - center).to_perpendicular();

                        let [center, a, b] = [center, a, b].map(|point| {
                            surface.geometry.point_from_local(point)
                        });

                        let origin = a;
                        let circle = Circle {
                            a: a - center,
                            b: b - center,
                        };

                        (origin, circle)
                    };

                    Handle::new(Curve {
                        geometry: CurveAnchored::from_origin_and_curve(
                            origin, circle,
                        ),
                    })
                }
                SketchSegment::Line { .. } => {
                    Handle::new(Curve::line_from_vertices([
                        &segment.start,
                        &next_segment.start,
                    ]))
                }
            };

            Handle::new(HalfEdge {
                curve,
                start: segment.start,
                is_internal,
            })
        })
        .collect()
}

fn check_that_half_edges_are_connected(
    half_edges: &[Handle<HalfEdge>],
    tolerance: Tolerance,
) {
    for (a, b) in half_edges.iter().circular_tuple_windows() {
        let b_start_on_a_curve = a.curve.geometry.project_point(b.start.point);
        let b_start_from_projected =
            a.curve.geometry.point_from_local(b_start_on_a_curve);

        let distance_between_original_and_from_projected =
            (b.start.point - b_start_from_projected).magnitude();

        assert!(
            distance_between_original_and_from_projected <= tolerance.inner(),
        );
    }
}
