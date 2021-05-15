use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::{Point, Vector3};

use crate::graphics;

pub struct Mesh {
    indices_by_vertex: HashMap<Vertex, graphics::Index>,

    vertices: Vec<Vertex>,
    triangles: Vec<[graphics::Index; 3]>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn triangle(
        &mut self,
        v0: impl Into<Point<f32, 3>>,
        v1: impl Into<Point<f32, 3>>,
        v2: impl Into<Point<f32, 3>>,
    ) {
        let v0 = v0.into();
        let v1 = v1.into();
        let v2 = v2.into();

        let normal = (v1 - v0).cross(&(v2 - v0)).normalize();
        let normal = normal.map(|v| R32::from_inner(v));

        let v0 = Vertex {
            position: v0.map(|c| c.into()).into(),
            normal,
        };
        let v1 = Vertex {
            position: v1.map(|c| c.into()).into(),
            normal,
        };
        let v2 = Vertex {
            position: v2.map(|c| c.into()).into(),
            normal,
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

        self.triangles.push([i0, i1, i2]);
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vertex> + '_ {
        self.vertices.iter().copied()
    }

    pub fn indices(&self) -> impl Iterator<Item = graphics::Index> + '_ {
        self.triangles.iter().flatten().copied()
    }

    pub fn triangles(&self) -> impl Iterator<Item = [Vertex; 3]> + '_ {
        self.triangles.iter().map(move |&[i0, i1, i2]| {
            [
                self.vertices[i0 as usize],
                self.vertices[i1 as usize],
                self.vertices[i2 as usize],
            ]
        })
    }

    pub fn invert_triangles(&mut self) {
        for [_, b, c] in &mut self.triangles {
            let tmp = *b;
            *b = *c;
            *c = tmp;
        }

        for vertex in &mut self.vertices {
            vertex.normal = -vertex.normal;
        }
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
    pub position: [R32; 3],
    pub normal: Vector3<R32>,
}

#[cfg(test)]
mod tests {
    use decorum::R32;
    use nalgebra::Point3;

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

        let normal = [0.0.into(), 0.0.into(), 1.0.into()].into();
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
}
