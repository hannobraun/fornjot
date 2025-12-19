use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    objects::{
        geometry::{Triangle, Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    store::{Index, Store},
};

pub fn vertex(
    vertex: Index<Vertex>,
    offset: impl Into<Vector<3>>,
    cache: &mut BTreeMap<Index<Vertex>, Index<Vertex>>,
    vertices: &mut Store<Vertex>,
) -> Index<Vertex> {
    if let Some(translated) = cache.get(&vertex).copied() {
        return translated;
    }

    let position = vertices[vertex].position;
    let translated = vertices.push(Vertex {
        position: position + offset.into(),
    });

    cache.insert(vertex, translated);

    translated
}

pub fn face(
    face: &Face,
    offset: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    _: &Faces,
) -> Face {
    use vertex as translate_vertex;

    let offset = offset.into();

    let mut vertex_cache = BTreeMap::new();

    let boundary = face
        .boundary
        .iter()
        .copied()
        .map(|half_edge| {
            half_edges.push(HalfEdge {
                boundary: half_edges[half_edge].boundary.map(|vertex| {
                    translate_vertex(
                        vertex,
                        offset,
                        &mut vertex_cache,
                        vertices,
                    )
                }),
            })
        })
        .collect();
    let triangles = face
        .triangles
        .iter()
        .copied()
        .map(|triangle| {
            triangles.push(
                Triangle {
                    vertices: triangles[triangle].vertices.map(|vertex| {
                        translate_vertex(
                            vertex,
                            offset,
                            &mut vertex_cache,
                            vertices,
                        )
                    }),
                },
                vertices,
            )
        })
        .collect();

    Face {
        boundary,
        triangles,
    }
}
