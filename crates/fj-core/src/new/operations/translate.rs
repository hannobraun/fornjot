use std::collections::BTreeMap;

use crate::{
    math::{Triangle, Vector},
    new::topology::{
        Edge, Face, HalfEdge, HalfFace, Handle, Store, Topology, Vertex,
    },
};

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
        topology: &mut Topology,
    ) -> Handle<HalfEdge> {
        let offset = offset.into();

        let edge = {
            let edge = self.edge(
                topology.half_edges[half_edge].edge,
                offset,
                topology,
            );
            topology.edges.push(edge)
        };
        let orientation = topology.half_edges[half_edge].orientation;

        topology.half_edges.push(HalfEdge { edge, orientation })
    }

    /// # Translate an edge
    pub fn edge(
        &mut self,
        edge: Handle<Edge>,
        offset: impl Into<Vector<3>>,
        topology: &mut Topology,
    ) -> Edge {
        let offset = offset.into();

        let edge = &topology.edges[edge];

        Edge {
            boundary: edge.boundary.map(|vertex| {
                self.vertex(vertex, offset, &mut topology.vertices)
            }),
            approx: edge.approx.iter().map(|&point| point + offset).collect(),
        }
    }

    /// # Translate a half-face
    pub fn half_face(
        &mut self,
        half_face: &HalfFace,
        offset: impl Into<Vector<3>>,
        topology: &mut Topology,
    ) -> HalfFace {
        let offset = offset.into();

        let boundary = half_face
            .boundary
            .iter()
            .copied()
            .map(|half_edge| self.half_edge(half_edge, offset, topology))
            .collect();
        let face = topology.faces.push(Face {
            approx: topology.faces[half_face.face]
                .approx
                .iter()
                .copied()
                .map(|triangle| self.triangle(triangle, offset))
                .collect(),
        });

        HalfFace {
            boundary,
            face,
            orientation: half_face.orientation,
        }
    }
}
