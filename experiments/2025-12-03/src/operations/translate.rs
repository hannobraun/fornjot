use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    objects::{
        geometry::{Triangle, Triangles, Vertex},
        topology::{Face, HalfEdge},
    },
    store::{Index, Store},
};

#[derive(Default)]
pub struct Translate {
    vertex: BTreeMap<Index<Vertex>, Index<Vertex>>,
}

impl Translate {
    pub fn vertex(
        &mut self,
        vertex: Index<Vertex>,
        offset: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
    ) -> Index<Vertex> {
        if let Some(translated) = self.vertex.get(&vertex).copied() {
            return translated;
        }

        let position = vertices[vertex].position;
        let translated = vertices.push(Vertex {
            position: position + offset.into(),
        });

        self.vertex.insert(vertex, translated);

        translated
    }

    pub fn triangle(
        &mut self,
        triangle: Index<Triangle>,
        offset: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
        triangles: &mut Triangles,
    ) -> Index<Triangle> {
        let offset = offset.into();

        triangles.push(
            Triangle {
                vertices: triangles[triangle]
                    .vertices
                    .map(|vertex| self.vertex(vertex, offset, vertices)),
            },
            vertices,
        )
    }
}

pub fn face(
    face: &Face,
    offset: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
) -> Face {
    let offset = offset.into();

    let mut translate = Translate::default();

    let boundary = face
        .boundary
        .iter()
        .copied()
        .map(|half_edge| {
            half_edges.push(HalfEdge {
                boundary: half_edges[half_edge]
                    .boundary
                    .map(|vertex| translate.vertex(vertex, offset, vertices)),
            })
        })
        .collect();
    let triangles = face
        .triangles
        .iter()
        .copied()
        .map(|triangle| {
            translate.triangle(triangle, offset, vertices, triangles)
        })
        .collect();

    Face {
        boundary,
        triangles,
    }
}
