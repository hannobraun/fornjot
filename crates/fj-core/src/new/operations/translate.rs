use std::collections::BTreeMap;

use fj_math::{Triangle, Vector};

use crate::new::topology::{Face, HalfEdge, Handle, Store, Vertex};

#[derive(Default)]
pub struct Translate {
    vertices: BTreeMap<Handle<Vertex>, Handle<Vertex>>,
}

impl Translate {
    pub fn new() -> Self {
        Self::default()
    }

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
        vertex: Handle<Vertex>,
        offset: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
    ) -> Handle<Vertex> {
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

    pub fn half_edge(
        &mut self,
        half_edge: Handle<HalfEdge>,
        offset: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Handle<HalfEdge> {
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

    pub fn face(
        &mut self,
        face: &Face,
        offset: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Face {
        let offset = offset.into();

        let boundary = face
            .boundary
            .iter()
            .copied()
            .map(|half_edge| {
                self.half_edge(half_edge, offset, vertices, half_edges)
            })
            .collect();
        let approx = face
            .approx
            .iter()
            .copied()
            .map(|triangle| self.triangle(triangle, offset))
            .collect();

        Face { boundary, approx }
    }
}
