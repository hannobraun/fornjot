use std::collections::BTreeMap;

use fj_math::{Triangle, Vector};

use crate::{
    store::{Index, Store},
    topology::{Face, HalfEdge, Vertex},
};

#[derive(Default)]
pub struct Translate {
    vertices: BTreeMap<Index<Vertex>, Index<Vertex>>,
}

impl Translate {
    pub fn triangle(
        &mut self,
        triangle: Triangle<3>,
        offset: impl Into<Vector<3>>,
    ) -> Triangle<3> {
        let offset = offset.into();

        Triangle {
            points: triangle.points.map(|point| point + offset),
        }
    }

    pub fn vertex(
        &mut self,
        vertex: Index<Vertex>,
        offset: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
    ) -> Index<Vertex> {
        let offset = offset.into();

        if let Some(translated) = self.vertices.get(&vertex).copied() {
            return translated;
        }

        let translated = vertices.push(Vertex {
            point: vertices[vertex].point + offset,
        });

        self.vertices.insert(vertex, translated);

        translated
    }

    pub fn half_eldge(
        &mut self,
        half_edge: Index<HalfEdge>,
        offset: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Index<HalfEdge> {
        let offset = offset.into();

        half_edges.push(HalfEdge {
            boundary: half_edges[half_edge]
                .boundary
                .map(|vertex| self.vertex(vertex, offset, vertices)),
            approx: half_edges[half_edge]
                .approx
                .iter()
                .copied()
                .map(|point| point + offset)
                .collect(),
        })
    }
}

pub fn face(
    face: &Face,
    offset: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    half_edges: &mut Store<HalfEdge>,
) -> Face {
    let offset = offset.into();

    let mut translate = Translate::default();

    let boundary = face
        .boundary
        .iter()
        .copied()
        .map(|half_edge| {
            translate.half_eldge(half_edge, offset, vertices, half_edges)
        })
        .collect();
    let approx = face
        .approx
        .iter()
        .copied()
        .map(|triangle| translate.triangle(triangle, offset))
        .collect();

    Face { boundary, approx }
}
