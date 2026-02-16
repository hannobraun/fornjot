use std::collections::BTreeMap;

use fj_math::{Triangle, Vector};

use crate::new::topology::{Face, HalfEdge, Handle, Store, Vertex};

/// # Translate primitives, given an offset
///
/// This operation is a placeholder for a more general "transform" operation.
#[derive(Default)]
pub struct Translate {
    vertices: BTreeMap<Handle<Vertex>, Handle<Vertex>>,
}

impl Translate {
    /// # Construct a new instance of `Translate`
    pub fn new() -> Self {
        Self::default()
    }

    /// # Translate a triangle
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

    /// # Translate a vertex
    ///
    /// Caches the result of the translation, and will return the same resulting
    /// vertex, if you call this method multiple times with the same input
    /// vertex.
    ///
    /// **This method returns a cached result depending only on the input
    /// vertex, disregarding the provided offset. This is a bug.**
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

    /// # Translate a half-edge
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

    /// # Translate a face
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
