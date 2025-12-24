use std::collections::BTreeMap;

use fj_math::{Point, Triangle, Vector};

use crate::{
    objects::{
        geometry::{Geometry, Vertex},
        topology::{Face, HalfEdge},
    },
    store::{Index, Store},
};

#[derive(Default)]
pub struct Translate {
    vertex: BTreeMap<Index<Vertex>, Index<Vertex>>,
}

impl Translate {
    pub fn point(
        &mut self,
        point: Index<Point<3>>,
        offset: impl Into<Vector<3>>,
        points: &mut Store<Point<3>>,
    ) -> Index<Point<3>> {
        points.push(points[point] + offset.into())
    }

    pub fn vertex(
        &mut self,
        vertex: Index<Vertex>,
        offset: impl Into<Vector<3>>,
        points: &mut Store<Point<3>>,
        vertices: &mut Store<Vertex>,
    ) -> Index<Vertex> {
        let offset = offset.into();

        if let Some(translated) = self.vertex.get(&vertex).copied() {
            return translated;
        }

        let translated = vertices.push(Vertex {
            point: self.point(vertices[vertex].point, offset, points),
        });

        self.vertex.insert(vertex, translated);

        translated
    }

    pub fn triangle(
        &mut self,
        triangle: Index<Triangle<3>>,
        offset: impl Into<Vector<3>>,
        geometry: &mut Geometry,
    ) -> Index<Triangle<3>> {
        let offset = offset.into();

        let translated = Triangle {
            points: geometry.triangles[triangle]
                .points
                .map(|point| point + offset),
        };

        geometry.triangles.push(translated, &geometry.points)
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
                    translate.vertex(
                        vertex,
                        offset,
                        &mut geometry.points,
                        &mut geometry.vertices,
                    )
                }),
            })
        })
        .collect();
    let triangles = face
        .triangles
        .iter()
        .copied()
        .map(|triangle| translate.triangle(triangle, offset, geometry))
        .collect();

    Face {
        boundary,
        triangles,
    }
}
