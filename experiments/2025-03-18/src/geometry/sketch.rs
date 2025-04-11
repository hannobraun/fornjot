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

    pub fn line_from(mut self, start: impl Into<Point<2>>) -> Self {
        let start = start.into();
        self.segments.push(SketchSegment::Line { start });
        self
    }

    pub fn to_face(&self, surface: Handle<Surface>) -> Face {
        let mut vertices_by_local_point: BTreeMap<_, Vec<_>> = BTreeMap::new();
        let vertices = self
            .segments
            .iter()
            .map(|segment| match segment {
                SketchSegment::Line { start } => start,
            })
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

        let mut coincident_vertices = BTreeSet::new();
        for vertices in vertices_by_local_point.into_values() {
            if vertices.len() > 1 {
                coincident_vertices.extend(vertices);
            }
        }

        let half_edges = vertices.into_iter().circular_tuple_windows().map(
            |(start, end)| {
                let curve =
                    Handle::new(Curve::line_from_vertices([&start, &end]));

                let is_internal = coincident_vertices.contains(&start)
                    && coincident_vertices.contains(&end);

                Handle::new(HalfEdge {
                    curve,
                    start,
                    is_internal,
                })
            },
        );

        Face::new(surface, half_edges, false)
    }
}

#[derive(Clone, Copy)]
enum SketchSegment {
    Line { start: Point<2> },
}
