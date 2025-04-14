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

        let half_edges = vertices.iter().map(|([start, end], is_internal)| {
            let curve = Handle::new(Curve::line_from_vertices([&start, &end]));

            Handle::new(HalfEdge {
                curve,
                start,
                is_internal,
            })
        });

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
    vertices: Vec<Handle<Vertex>>,
    coincident_vertices: BTreeSet<Handle<Vertex>>,
}

impl VerticesFromSegments {
    fn new(segments: &[SketchSegment], surface: &Handle<Surface>) -> Self {
        let mut vertices_by_local_point: BTreeMap<_, Vec<_>> = BTreeMap::new();
        let mut coincident_vertices = BTreeSet::new();

        let vertices = segments
            .iter()
            .map(SketchSegment::start)
            .copied()
            .map(|point_local| {
                let point_global =
                    surface.geometry.point_from_local(point_local);
                let vertex = Handle::new(Vertex::new(point_global));

                vertices_by_local_point
                    .entry(point_local)
                    .or_default()
                    .push(vertex.clone());

                vertex
            })
            .collect::<Vec<_>>();

        for vertices in vertices_by_local_point.into_values() {
            if vertices.len() > 1 {
                coincident_vertices.extend(vertices);
            }
        }

        VerticesFromSegments {
            vertices,
            coincident_vertices,
        }
    }

    fn iter(&self) -> impl Iterator<Item = ([Handle<Vertex>; 2], bool)> {
        self.vertices.iter().cloned().circular_tuple_windows().map(
            |(start, end)| {
                let [start_is_coincident, end_is_coincident] = [&start, &end]
                    .map(|vertex| self.coincident_vertices.contains(vertex));
                let is_internal = start_is_coincident && end_is_coincident;

                ([start, end], is_internal)
            },
        )
    }
}
