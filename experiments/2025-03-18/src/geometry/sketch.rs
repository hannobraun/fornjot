use std::collections::{BTreeMap, BTreeSet};

use fj_math::Point;
use itertools::Itertools;

use crate::{
    handle::Handle,
    topology::{
        curve::Curve, face::Face, half_edge::HalfEdge, surface::Surface,
        vertex::Vertex,
    },
};

pub struct Sketch {
    segments: Vec<SketchSegment>,
}

impl Sketch {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn arc_from(mut self, start: impl Into<Point<2>>) -> Self {
        let start = start.into();
        self.segments.push(SketchSegment::Arc { start });
        self
    }

    pub fn line_from(mut self, start: impl Into<Point<2>>) -> Self {
        let start = start.into();
        self.segments.push(SketchSegment::Line { start });
        self
    }

    pub fn to_face(&self, surface: Handle<Surface>) -> Face {
        let vertices = VerticesFromSegments::new(&self.segments, &surface);

        let half_edges = vertices.iter().map(
            |([(start_vertex, segment), (end_vertex, _)], is_internal)| {
                let curve = match segment {
                    SketchSegment::Arc { .. } => {
                        // We are creating a line here, temporarily, while
                        // support for arcs is being implemented.
                        Handle::new(Curve::line_from_vertices([
                            &start_vertex,
                            &end_vertex,
                        ]))
                    }
                    SketchSegment::Line { .. } => {
                        Handle::new(Curve::line_from_vertices([
                            &start_vertex,
                            &end_vertex,
                        ]))
                    }
                };

                Handle::new(HalfEdge {
                    curve,
                    start: start_vertex,
                    is_internal,
                })
            },
        );

        Face::new(surface, half_edges, false)
    }
}

#[derive(Clone, Copy)]
enum SketchSegment {
    Arc { start: Point<2> },
    Line { start: Point<2> },
}

impl SketchSegment {
    fn start(&self) -> &Point<2> {
        match self {
            SketchSegment::Arc { start } => start,
            SketchSegment::Line { start } => start,
        }
    }
}

struct VerticesFromSegments {
    segments_with_start_vertex: Vec<(SketchSegment, Handle<Vertex>)>,
    coincident_vertices: BTreeSet<Handle<Vertex>>,
}

impl VerticesFromSegments {
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

        VerticesFromSegments {
            segments_with_start_vertex,
            coincident_vertices,
        }
    }

    fn iter(
        &self,
    ) -> impl Iterator<Item = ([(Handle<Vertex>, SketchSegment); 2], bool)>
    {
        self.segments_with_start_vertex
            .iter()
            .cloned()
            .circular_tuple_windows()
            .map(|((segment, start), (next_segment, end))| {
                let [start_is_coincident, end_is_coincident] = [&start, &end]
                    .map(|vertex| self.coincident_vertices.contains(vertex));
                let is_internal = start_is_coincident && end_is_coincident;

                ([(start, segment), (end, next_segment)], is_internal)
            })
    }
}
