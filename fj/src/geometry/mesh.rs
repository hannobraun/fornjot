use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::Vector3;

use crate::{geometry::shapes::Point, graphics};

pub struct Mesh {
    indices_by_vertex: HashMap<Vertex, graphics::Index>,

    vertices: Vec<Vertex>,
    indices: Vec<graphics::Index>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn triangle(
        &mut self,
        v0: impl Into<Point<3>>,
        v1: impl Into<Point<3>>,
        v2: impl Into<Point<3>>,
    ) {
        let v0: nalgebra::Point<f32, 3> = v0.into().into();
        let v1: nalgebra::Point<f32, 3> = v1.into().into();
        let v2: nalgebra::Point<f32, 3> = v2.into().into();

        let normal = (v1 - v0).cross(&(v2 - v0)).normalize();
        let normal = normal.map(|v| R32::from_inner(v));

        let v0 = Vertex {
            position: v0.into(),
            normal,
        };
        let v1 = Vertex {
            position: v1.into(),
            normal,
        };
        let v2 = Vertex {
            position: v2.into(),
            normal,
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vertex> + '_ {
        self.vertices.iter().copied()
    }

    pub fn indices(&self) -> impl Iterator<Item = graphics::Index> + '_ {
        self.indices.iter().copied()
    }

    pub fn into_graphics_mesh(self) -> graphics::Mesh {
        let vertices = self
            .vertices()
            .map(|vertex| graphics::Vertex {
                position: [
                    vertex.position[0].into_inner(),
                    vertex.position[1].into_inner(),
                    vertex.position[2].into_inner(),
                ],
                normal: [
                    vertex.normal[0].into_inner(),
                    vertex.normal[1].into_inner(),
                    vertex.normal[2].into_inner(),
                ],
            })
            .collect();
        let indices = self.indices().collect();

        graphics::Mesh { vertices, indices }
    }

    fn index_for_vertex(&mut self, vertex: Vertex) -> graphics::Index {
        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Vertex {
    pub position: Point<3>,
    pub normal: Vector3<R32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Index(u16);

#[cfg(test)]
mod tests {
    use decorum::R32;
    use nalgebra::{Point3, Vector3};

    use super::{Mesh, Vertex};

    #[test]
    fn mesh_should_convert_triangle_into_vertices_and_indices() {
        let mut mesh = Mesh::new();

        let v0 = [0.0, 0.0, 0.0];
        let v1 = [0.5, 0.0, 0.0];
        let v2 = [0.0, 0.5, 0.0];

        mesh.triangle(v0, v1, v2);

        let mut vertices: Vec<Vertex> = Vec::new();
        vertices.extend(mesh.vertices());

        let mut indexed_vertices = Vec::new();
        for i in mesh.indices() {
            indexed_vertices.push(vertices[i as usize]);
        }

        let normal = Vector3::new(
            R32::from_inner(0.0),
            R32::from_inner(0.0),
            R32::from_inner(1.0),
        );
        assert_eq!(
            indexed_vertices,
            vec![
                Vertex {
                    position: Point3::from(
                        Point3::from(v0).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
                Vertex {
                    position: Point3::from(
                        Point3::from(v1).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
                Vertex {
                    position: Point3::from(
                        Point3::from(v2).coords.map(|f| R32::from_inner(f))
                    )
                    .into(),
                    normal,
                },
            ]
        );
    }

    // TASK: Add method that inverts triangles of a mesh.
    // TASK: Add method that merges another mesh into the mesh.
}
