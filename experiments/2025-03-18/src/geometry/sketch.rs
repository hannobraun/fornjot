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
    pub points: Vec<Point<2>>,
}

impl Sketch {
    pub fn to_face(&self, surface: Handle<Surface>) -> Face {
        let mut vertices_by_local_point: BTreeMap<_, Vec<_>> = BTreeMap::new();
        let vertices = self
            .points
            .iter()
            .copied()
            .map(|point| {
                let point = surface.geometry.point_from_local(point);
                let vertex = Handle::new(Vertex::new(point));

                vertices_by_local_point
                    .entry(point)
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
                let curve = Handle::new(Curve {});

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

impl<I, P> From<I> for Sketch
where
    I: IntoIterator<Item = P>,
    P: Into<Point<2>>,
{
    fn from(points: I) -> Self {
        let points = points.into_iter().map(Into::into).collect();
        Self { points }
    }
}
