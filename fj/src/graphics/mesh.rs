use std::{collections::HashMap, convert::TryInto, iter};

use euclid::default::Point3D;

use super::vertices::{Array3, Index, Vertex};

pub struct Mesh {
    positions: Vec<Array3>,
    indices_by_vertex: HashMap<Vertex, Index>,

    vertices: Vec<Vertex>,
    indices: Vec<Index>,
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

    pub fn vertex(&mut self, vertex: [f32; 3]) -> I {
        let i = self.positions.len();
        self.positions.push(Array3::new(vertex));
        I(i)
    }

    pub fn triangle(&mut self, i0: I, i1: I, i2: I) {
        let p0 = self.positions[i0.0];
        let p1 = self.positions[i1.0];
        let p2 = self.positions[i2.0];

        let normal = (Point3D::from(p1.0) - Point3D::from(p0.0))
            .cross(Point3D::from(p2.0) - Point3D::from(p0.0))
            .to_array();

        let v0 = Vertex {
            position: p0,
            normal: Array3(normal),
        };
        let v1 = Vertex {
            position: p1,
            normal: Array3(normal),
        };
        let v2 = Vertex {
            position: p2,
            normal: Array3(normal),
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }

    pub fn triangles(&self) -> impl Iterator<Item = [[f32; 3]; 3]> + '_ {
        let mut indices = self.indices().iter();

        iter::from_fn(move || {
            let &i0 = indices.next()?;
            let &i1 = indices.next()?;
            let &i2 = indices.next()?;

            let v0 = self.vertices[i0 as usize].position.into_f32_array();
            let v1 = self.vertices[i1 as usize].position.into_f32_array();
            let v2 = self.vertices[i2 as usize].position.into_f32_array();

            Some([v0, v1, v2])
        })
    }

    fn index_for_vertex(&mut self, vertex: Vertex) -> Index {
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
pub struct I(usize);

#[cfg(test)]
mod tests {
    use crate::graphics::vertices::{Array3, Vertex};

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

        let triangles: Vec<_> = mesh.triangles().collect();
        assert_eq!(triangles, vec![[v0, v1, v2], [v0, v2, v1]]);
    }
}
