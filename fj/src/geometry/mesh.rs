use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::Point3;

use crate::{
    geometry::{Triangle, Triangles},
    graphics,
};

pub struct Mesh {
    positions: Vec<graphics::Array3>,
    indices_by_vertex: HashMap<graphics::Vertex, graphics::Index>,

    vertices: Vec<graphics::Vertex>,
    indices: Vec<graphics::Index>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn vertex(&mut self, vertex: [f32; 3]) -> Index {
        let i = self.positions.len();
        self.positions.push(graphics::Array3::new(vertex));
        Index(i)
    }

    pub fn triangle(&mut self, i0: Index, i1: Index, i2: Index) {
        let p0 = self.positions[i0.0];
        let p1 = self.positions[i1.0];
        let p2 = self.positions[i2.0];

        let normal = (Point3::from(p1.0) - Point3::from(p0.0))
            .cross(&(Point3::from(p2.0) - Point3::from(p0.0)));

        let mut normal_array = [R32::from_inner(0.0); 3];
        normal_array.copy_from_slice(normal.data.as_slice());

        let v0 = graphics::Vertex {
            position: p0,
            normal: graphics::Array3(normal_array),
        };
        let v1 = graphics::Vertex {
            position: p1,
            normal: graphics::Array3(normal_array),
        };
        let v2 = graphics::Vertex {
            position: p2,
            normal: graphics::Array3(normal_array),
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn vertices(&self) -> &[graphics::Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[graphics::Index] {
        self.indices.as_slice()
    }

    pub fn into_graphics_mesh(self) -> graphics::Mesh {
        let vertices = self.vertices;
        let indices = self.indices;

        graphics::Mesh { vertices, indices }
    }

    pub fn triangles(&self) -> Triangles {
        let mut indices = self.indices().iter();

        let mut next_triangle = || {
            let &i0 = indices.next()?;
            let &i1 = indices.next()?;
            let &i2 = indices.next()?;

            let v0 = self.vertices[i0 as usize].position.into_f32_array();
            let v1 = self.vertices[i1 as usize].position.into_f32_array();
            let v2 = self.vertices[i2 as usize].position.into_f32_array();

            Some(Triangle::new(v0, v1, v2))
        };

        let mut triangles = Vec::new();

        while let Some(triangle) = next_triangle() {
            triangles.push(triangle);
        }

        Triangles(triangles)
    }

    fn index_for_vertex(
        &mut self,
        vertex: graphics::Vertex,
    ) -> graphics::Index {
        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}

#[derive(Clone, Copy)]
pub struct Index(usize);

#[cfg(test)]
mod tests {
    use crate::{
        geometry::Triangle,
        graphics::{Array3, Vertex},
    };

    use super::Mesh;

    #[test]
    fn mesh_should_convert_triangle_into_vertices_and_indices() {
        let mut mesh = Mesh::new();

        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let i0 = mesh.vertex(v0);
        let i1 = mesh.vertex(v1);
        let i2 = mesh.vertex(v2);

        mesh.triangle(i0, i1, i2);

        let mut vertices = Vec::new();
        for &i in mesh.indices() {
            vertices.push(mesh.vertices()[i as usize]);
        }

        assert_eq!(
            vertices,
            vec![
                Vertex {
                    position: Array3::new(v0),
                    normal: Array3::new([0.0, 0.0, 1.0])
                },
                Vertex {
                    position: Array3::new(v1),
                    normal: Array3::new([0.0, 0.0, 1.0])
                },
                Vertex {
                    position: Array3::new(v2),
                    normal: Array3::new([0.0, 0.0, 1.0])
                },
            ]
        );
    }

    #[test]
    fn mesh_should_return_triangles() {
        let mut mesh = Mesh::new();

        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let i0 = mesh.vertex(v0);
        let i1 = mesh.vertex(v1);
        let i2 = mesh.vertex(v2);

        mesh.triangle(i0, i1, i2);
        mesh.triangle(i0, i2, i1);

        let triangles = mesh.triangles();
        assert_eq!(
            triangles.0,
            vec![Triangle::new(v0, v1, v2), Triangle::new(v0, v2, v1)]
        );
    }
}
