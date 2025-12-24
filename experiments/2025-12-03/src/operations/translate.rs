use std::collections::BTreeMap;

use fj_math::{Triangle, Vector};

use crate::{
    objects::{
        geometry::{Geometry, Vertex},
        topology::{Face, HalfEdge},
    },
    store::{Index, Store},
};

#[derive(Default)]
pub struct Translate {
    vertices: BTreeMap<Index<Vertex>, Index<Vertex>>,
}

impl Translate {
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
}

pub fn face(
    face: &Face,
    offset: impl Into<Vector<3>>,
    geometry: &mut Geometry,
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
                boundary: half_edges[half_edge].boundary.map(|vertex| {
                    translate.vertex(vertex, offset, &mut geometry.vertices)
                }),
            })
        })
        .collect();
    let triangles = face
        .triangles
        .iter()
        .copied()
        .map(|triangle| {
            let triangle =
                translate.triangle(geometry.triangles[triangle], offset);
            geometry.triangles.push(triangle)
        })
        .collect();

    Face {
        boundary,
        triangles,
    }
}
