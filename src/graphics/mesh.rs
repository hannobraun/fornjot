use bytemuck::{Pod, Zeroable};

use crate::{
    geometry::faces::Triangle,
    mesh::{HashVector, Index, MeshMaker},
};

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Mesh {
    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}

impl From<&Vec<Triangle>> for Mesh {
    fn from(triangles: &Vec<Triangle>) -> Self {
        let mut mesh = MeshMaker::new();

        for triangle in triangles {
            let [a, b, c] = triangle.0;

            let normal = (b - a).cross(&(c - a)).normalize();

            let a = HashVector::from(a);
            let b = HashVector::from(b);
            let c = HashVector::from(c);

            let normal = HashVector::from(normal);

            mesh.push((a, normal));
            mesh.push((b, normal));
            mesh.push((c, normal));
        }

        let vertices = mesh
            .vertices()
            .map(|(vertex, normal)| Vertex {
                position: vertex.into(),
                normal: normal.into(),
                color: [1.0, 0.0, 0.0, 1.0],
            })
            .collect();

        let indices = mesh.indices().collect();

        Self { vertices, indices }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}
